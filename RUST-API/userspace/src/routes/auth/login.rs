use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::user::LoginRequest;
use crate::utils::hashing::verify_password;

#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(
    pool: &State<PgPool>,
    login_data: Json<LoginRequest>,
) -> Result<Json<Value>, Status> {
    let credentials = login_data.into_inner();

    if let Err(_) = credentials.validate() {
        return Err(Status::BadRequest);
    }

    let user = sqlx::query!(
        r#"
        SELECT id, email, username, password_hash, created_at
        FROM users 
        WHERE email = $1
        "#,
        credentials.email
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::Unauthorized)?;

    if !verify_password(&credentials.password, &user.password_hash)
        .map_err(|_| Status::InternalServerError)? {
        return Err(Status::Unauthorized);
    }

    sqlx::query!(
        r#"
        UPDATE users 
        SET last_login = CURRENT_TIMESTAMP 
        WHERE id = $1
        "#,
        user.id
    )
    .execute(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(Json(json!({
        "id": user.id,
        "email": user.email,
        "username": user.username,
        "created_at": user.created_at
    })))
}