use rocket::{post, serde::json::Json, State};
use crate::services::auth::AuthService;
use serde::Deserialize;

use crate::models::user::User;

#[derive(Deserialize)]
pub struct NewUser {
    username: String,
    email: String,
}

#[post("/user", data = "<user_data>")]
pub async fn create_user(
    user_data: Json<NewUser>, 
    auth_service: &State<AuthService>
) -> Result<Json<User>, String> {
    let user_data = user_data.into_inner();
    
    auth_service.create_user(user_data.username, user_data.email)
        .await
        .map(Json)
        .map_err(|e| format!("Failed to create user: {}", e))
}

#[get("/user/<id>")]
pub async fn get_user(id: String, auth_service: &State<AuthService>) -> Result<Json<User>, String> {
    // Parse the ID string into an ObjectId
    match mongodb::bson::oid::ObjectId::parse_str(id) {
        Ok(id_as_oid) => {
            // Call the method directly without needing a Mutex
            match auth_service.get_user_by_id(id_as_oid).await {
                Ok(Some(user)) => {
                    // Return the user wrapped in Json for Rocket to serialize as JSON
                    Ok(Json(user))
                }
                Ok(None) => Err("User not found".to_string()),
                Err(_) => Err("Error fetching user from database".to_string()),
            }
        }
        Err(_) => Err("Invalid ObjectId format".to_string()),
    }
}

#[put("/user/<id>", data = "<update_data>")]
pub async fn update_user(
    id: String, 
    update_data: Json<mongodb::bson::Document>,
    auth_service: &State<AuthService>
) -> Result<Json<User>, String> {
    match mongodb::bson::oid::ObjectId::parse_str(id) {
        Ok(id_as_oid) => {
            match auth_service.update_user_by_id(id_as_oid, update_data.into_inner()).await {
                Ok(Some(updated_user)) => Ok(Json(updated_user)),
                Ok(None) => Err("User not found".to_string()),
                Err(_) => Err("Error updating user".to_string()),
            }
        }
        Err(_) => Err("Invalid ObjectId format".to_string()),
    }
}