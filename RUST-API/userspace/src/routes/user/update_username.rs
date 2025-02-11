use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::models::user::UpdateUser;
use crate::utils::logger::{LogBuilder, LogAction, log_action};

#[put("/username", format="json", data="<username_data>")]
pub async fn update_username(
    user: AuthenticatedUser, 
    pool: &State<PgPool>, 
    username_data: Json<UpdateUser>
) -> Result<Json<Value>, Status> {
    let username = username_data.into_inner();

    if username.validate().is_err() {
        return Err(Status::BadRequest);
    }

    // Get current username first
    let current_user = sqlx::query!(
        "SELECT username FROM users WHERE id = $1",
        user.user_id
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::NotFound)?;

    let result = sqlx::query!(
        "UPDATE users SET username = $1 WHERE id = $2",
        username.username, 
        user.user_id
    )
    .execute(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    if result.rows_affected() == 0 {
        return Err(Status::NotFound);
    }

    // Enhanced logging
    let log = LogBuilder::new(LogAction::Update, "user")
        .with_user(user.user_id)
        .with_resource_id(user.user_id.to_string())
        .with_previous_state(&json!({
            "username": current_user.username
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_new_state(&json!({
            "username": username.username
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "Username successfully updated!"
    })))
}