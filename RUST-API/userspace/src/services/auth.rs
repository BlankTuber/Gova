use mongodb::{Collection, Database};
use mongodb::bson::doc;

use crate::models::user::{User, UserRole, UserStatus};

use uuid::Uuid;

pub struct AuthService {
    user_collection: Collection<User>,
    db: Database
}

impl AuthService {
    pub fn new(db: Database) -> Self {
        AuthService {
            user_collection: db.collection("users"),
            db,
        }
    }

    pub async fn create_user(&self, username: String, email: String) -> Result<Uuid, mongodb::error::Error> {
        let user = User {
            id: Uuid::new_v4(),
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

        self.user_collection
        .insert_one(&user)
        .await?;

        Ok(user.id)
    }
}