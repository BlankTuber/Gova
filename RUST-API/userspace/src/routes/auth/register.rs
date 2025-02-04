use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::user::RegisterRequest;
use crate::utils::hashing::hash_password;

#[post("/register", format = "json", data = "<user_data>")]
pub async fn register(
    pool: &State<PgPool>,
    user_data: Json<RegisterRequest>,
) -> Result<Json<Value>, Status> {
    let user = user_data.into_inner();

    if let Err(_) = user.validate() {
        return Err(Status::BadRequest);
    }

    let password_hash = hash_password(&user.password)
        .map_err(|_| Status::InternalServerError)?;

    let result = sqlx::query!(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, created_at
        "#,
        user.email,
        user.username,
        password_hash
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(err) if err.is_unique_violation() => Status::Conflict,
        _ => Status::InternalServerError,
    })?;

    Ok(Json(json!({
        "id": result.id,
        "email": result.email,
        "username": result.username,
        "created_at": result.created_at
    })))
}