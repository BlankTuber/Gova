use rocket::serde::json::{Json, json};
use rocket::time::Duration;
use rocket::State;
use rocket::http::{Status, Cookie, CookieJar, SameSite};
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::user::RegisterRequest;
use crate::utils::hashing::hash_password;
use crate::utils::create_jwt::create_token;
use crate::utils::logger::{log_action, LogAction, LogBuilder};

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

    // Start a transaction since we're doing multiple operations
    let mut tx = pool.begin().await
        .map_err(|_| Status::InternalServerError)?;

    let result = sqlx::query!(
        r#"
        INSERT INTO users (email, username, password_hash, last_login)
        VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
        RETURNING id, email, username, created_at
        "#,
        user.email,
        user.username,
        password_hash,
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(err) if err.is_unique_violation() => Status::Conflict,
        _ => Status::InternalServerError,
    })?;

    // Create minimal user profile
    let profile_result = sqlx::query!(
        r#"
        INSERT INTO user_profiles (user_id, created_at, updated_at)
        VALUES ($1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        RETURNING id
        "#,
        result.id  // Using the ID from the newly created user
    )
    .fetch_one(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Commit the transaction
    tx.commit().await
        .map_err(|_| Status::InternalServerError)?;

    // Generate a JWT for the authenticated user
    let token = create_token(result.id)
        .map_err(|_| Status::InternalServerError)?;

    // Create a secure, HTTP-only cookie to store the JWT
    let mut cookie = Cookie::new("auth_token", token);
    cookie.set_path("/");
    cookie.set_secure(true);
    cookie.set_http_only(true);
    cookie.set_same_site(SameSite::Strict);
    cookie.set_max_age(Duration::days(7));

    // Add the cookie to the response
    cookies.add_private(cookie);

    // Log successful registration and profile creation
    let log = LogBuilder::new(LogAction::Create, "user")
        .with_user(result.id)
        .with_resource_id(result.id.to_string())
        .with_new_state(&json!({
            "email": result.email,
            "username": result.username,
            "created_at": result.created_at,
            "profile_id": profile_result.id
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "registration_timestamp": chrono::Utc::now().to_rfc3339(),
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "User registered with profile and logged in successfully!"
    })))
}