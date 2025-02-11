use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::models::user::UpdateUser;
use crate::utils::logger::{LogBuilder, LogAction, log_action};

#[put("/email", format="json", data="<email_data>")]
pub async fn update_email(
    user: AuthenticatedUser, 
    pool: &State<PgPool>, 
    email_data: Json<UpdateUser>
) -> Result<Json<Value>, Status> {
    let email = email_data.into_inner();

    // Validate the incoming email data
    if email.validate().is_err() {
        return Err(Status::BadRequest);
    }

    // First, get the current email
    let current_user = sqlx::query!(
        "SELECT email FROM users WHERE id = $1",
        user.user_id
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::NotFound)?;

    // Perform the update
    let result = sqlx::query!(
        r#"
            UPDATE users
            SET email = $1
            WHERE id = $2
        "#,
        email.email, 
        user.user_id
    )
    .execute(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Check if any rows were updated
    if result.rows_affected() == 0 {
        return Err(Status::NotFound);
    }

    // Create detailed log entry
    let log = LogBuilder::new(LogAction::Update, "user")
        .with_user(user.user_id)
        .with_resource_id(user.user_id.to_string())
        .with_previous_state(&json!({
            "email": current_user.email
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_new_state(&json!({
            "email": email.email
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "ip_address": "TODO: Add IP address capture"
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "Email successfully updated!"
    })))
}