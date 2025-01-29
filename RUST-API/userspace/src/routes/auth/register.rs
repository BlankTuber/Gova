use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Json;

use crate::config::database::DbPool;
use crate::controllers::auth::register::register_user;
use crate::models::user::NewUser;

#[post("/register", format = "json", data = "<user_data>")]
pub async fn register(user_data: Json<NewUser>, pool: &State<DbPool>) -> Result<Status, Status> {
    match register_user(user_data.into_inner(), &pool).await {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::InternalServerError),
    }
}