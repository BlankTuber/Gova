use mongodb::{Collection, Database};
use mongodb::bson::{doc, Document};

use crate::models::user::{User, UserRole, UserStatus};

pub struct AuthService {
    user_collection: Collection<User>
}

impl AuthService {
    pub fn new(db: Database) -> Self {
        AuthService {
            user_collection: db.collection("users")
        }
    }

    pub async fn create_user(&self, username: String, email: String) -> Result<User, mongodb::error::Error> {
        let mut user = User {
            id: None,
            username,
            email,
            role: UserRole::User,
            status: UserStatus::Active,
            profile: None,
        };

        let existing_user = self.user_collection
            .find_one(
                doc! { 
                    "$or": [
                        { "username": &user.username },
                        { "email": &user.email }
                    ]}).await?;

        if existing_user.is_some() {
            return Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists, 
                "Username or email already exists"
            )));
        }

        let result = self.user_collection
            .insert_one(&user)
            .await?;

        user.id = Some(result.inserted_id.as_object_id().unwrap().to_hex()); 

        Ok(user) //sends user info but not the mongodb entry
    }

    pub async fn get_user_by_id(&self, id: mongodb::bson::oid::ObjectId) -> Result<Option<User>, mongodb::error::Error> {
        let filter = doc! {"_id": id};
        let mut user = self.user_collection.find_one(filter).await?;
        
        if let Some(ref mut u) = user {
            u.id = Some(id.to_string());
        }
        
        Ok(user)
    }

    fn filter_sensitive_fields(mut doc: Document) -> Document {
        let sensitive_fields = ["role", "status", "id", "_id", "profile"];
        for field in sensitive_fields.iter() {
            doc.remove(*field);
        }
        doc
    }

    pub async fn update_user_by_id(
        &self,
        id: mongodb::bson::oid::ObjectId,
        update_data: Document
    ) -> Result<Option<User>, mongodb::error::Error> {
        let query = doc! {"_id": id};
        
        // Filter out sensitive fields
        let filtered_update_data = AuthService::filter_sensitive_fields(update_data);
        
        let update = doc! {"$set": filtered_update_data};
        
        self.user_collection
            .update_one(query, update)
            .await?;
    
        let user = self.get_user_by_id(id).await?;
    
        Ok(user)
    }
}