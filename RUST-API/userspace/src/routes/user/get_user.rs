use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;

use crate::middleware::verify_jwt::AuthenticatedUser;

#[get("/")]
pub async fn get_user(
    pool: &State<PgPool>,
    cookie_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    let user = sqlx::query!(
        r#"
        SELECT id, username, email, created_at, updated_at, last_login
        FROM users
        WHERE id = $1
        "#, cookie_user.user_id
    )
    .fetch_optional(pool.inner()) // Use fetch_optional instead of fetch_all
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Check if user exists
    if let Some(user) = user {
        Ok(Json(json!({
            "message": "User found!",
            "user": {
                "id": user.id,
                "username": user.username,
                "email": user.email,
                "created_at": user.created_at,
                "updated_at": user.updated_at,
                "last_login": user.last_login
            }
        })))
    } else {
        Err(Status::NotFound) // Return 404 if user is not found
    }
}
