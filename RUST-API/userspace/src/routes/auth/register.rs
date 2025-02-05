use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::{Status, Cookie, CookieJar, SameSite};
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::user::RegisterRequest;
use crate::utils::hashing::hash_password;
use crate::utils::create_jwt::create_token;

#[post("/register", format = "json", data = "<user_data>")]
pub async fn register(
    cookies: &CookieJar<'_>,
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

    // Generate a JWT for the authenticated user
    let token = create_token(result.id)
        .map_err(|_| Status::InternalServerError)?;

    // Create a secure, HTTP-only cookie to store the JWT
    let mut cookie = Cookie::new("auth_token", token);
    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);

    // Add the cookie to the response
    cookies.add_private(cookie);

    Ok(Json(json!({
        "message": "User registered and logged in successfully!"
    })))
}