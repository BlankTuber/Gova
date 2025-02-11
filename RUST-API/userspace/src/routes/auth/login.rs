use rocket::serde::json::{Json, json};
use rocket::time::Duration;
use rocket::State;
use rocket::http::{Status, Cookie, CookieJar, SameSite};
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;
use crate::models::user::LoginRequest;
use crate::utils::hashing::verify_password;
use crate::utils::create_jwt::create_token;

use crate::utils::logger::{log_action, LogAction, LogBuilder};

#[post("/login", format = "json", data = "<login_data>")]
pub async fn login(
    cookies: &CookieJar<'_>,
    pool: &State<PgPool>,
    login_data: Json<LoginRequest>,
) -> Result<Json<Value>, Status> {
    let credentials = login_data.into_inner();

    if let Err(_) = credentials.validate() {
        return Err(Status::BadRequest);
    }

    // Fetch the user from the database
    let user = sqlx::query!(
        r#"
        SELECT id, email, username, password_hash, created_at, last_login
        FROM users 
        WHERE email = $1
        "#,
        credentials.email
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::Unauthorized)?;

    // Verify the password
    if !verify_password(&credentials.password, &user.password_hash)
        .map_err(|_| Status::InternalServerError)? {
        
        // Log failed login attempt
        let failed_log = LogBuilder::new(LogAction::Custom("login_failed".to_string()), "auth")
            .with_user(user.id)
            .with_additional_details(&json!({
                "email": credentials.email,
                "timestamp": chrono::Utc::now().to_rfc3339(),
                "failure_reason": "invalid_password"
            }))
            .map_err(|_| Status::InternalServerError)?
            .build();

        let _ = log_action(pool.inner(), &failed_log).await;
        return Err(Status::Unauthorized);
    }

    // Update last login
    sqlx::query!(
        "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = $1",
        user.id
    )
    .execute(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Generate a JWT for the authenticated user
    let token = create_token(user.id)
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

    // Log successful login
    let log = LogBuilder::new(LogAction::Custom("login_successful".to_string()), "auth")
        .with_user(user.id)
        .with_resource_id(user.id.to_string())
        .with_previous_state(&json!({
            "last_login": user.last_login,
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "login_info": {
                "email": user.email,
                "username": user.username,
                "timestamp": chrono::Utc::now().to_rfc3339(),
            },
            "account_info": {
                "account_created_at": user.created_at,
            }
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "Logged in successfully"
    })))
}