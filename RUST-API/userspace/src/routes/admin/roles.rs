use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::role::CreateRole;
use crate::utils::auth::is_admin;
use crate::middleware::verify_jwt::AuthenticatedUser;

#[post("/role", format = "json", data = "<role_data>")]
pub async fn make_role(
    pool: &State<PgPool>,
    user: AuthenticatedUser,
    role_data: Json<CreateRole>,
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), user.user_id).await? {
        return Err(Status::Forbidden); // Block access if not Admin
    }

    let role = role_data.into_inner();

    if let Err(_) = role.validate() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO roles (name)
        VALUES ($1)
        RETURNING id, name, created_at
        "#,
        role.name,
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