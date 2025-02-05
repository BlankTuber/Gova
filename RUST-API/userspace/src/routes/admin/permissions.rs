use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::permission::CreatePermission;
use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::utils::auth::is_admin;

#[post("/permission", format = "json", data = "<permission_data>")]
pub async fn make_permission(
    pool: &State<PgPool>,
    user: AuthenticatedUser,
    permission_data: Json<CreatePermission>,
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), user.user_id).await? {
        return Err(Status::Forbidden); // Block access if not Admin
    }

    let permission = permission_data.into_inner();

    if let Err(_) = permission.validate() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO roles (name)
        VALUES ($1)
        RETURNING id, name, created_at
        "#,
        permission.name,
    )
    .fetch_one(pool.inner())
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(err) if err.is_unique_violation() => Status::Conflict,
        _ => Status::InternalServerError,
    })?;

    Ok(Json(json!({
        "id": result.id,
        "name": result.name,
        "created_at": result.created_at
    })))
}