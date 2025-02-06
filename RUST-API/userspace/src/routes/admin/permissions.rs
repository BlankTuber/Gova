use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::log::CreateLog;
use crate::models::permission::CreatePermission;
use crate::models::permission::DeletePermission;
use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::utils::auth::is_admin;
use crate::utils::logger::log_action;

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

    // Log successful
    let log = CreateLog {
        user_id: Some(admin_user.user_id),
        action: "permission_created_successfully".to_string(),
        details: json!({
            "permission_id": result.id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "id": result.id,
        "name": result.name,
        "created_at": result.created_at
    })))
}

#[get("/permissions")]
pub async fn get_all_permissions(
    pool: &State<PgPool>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let permissions = sqlx::query!(
        r#"
        SELECT id, name FROM permissions
        "#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Convert the query result into JSON
    let permissions_json: Vec<Value> = permissions.iter().map(|permission| {
        json!({
            "id": permission.id,
            "name": permission.name,
        })
    }).collect();

    Ok(Json(json!({
        "message": "Found permissions!",
        "permissions": permissions_json
    })))
}

#[delete("/permission", format = "json", data = "<permission_data>")]
pub async fn delete_permission(
    pool: &State<PgPool>,
    user: AuthenticatedUser,
    permission_data: Json<DeletePermission>,
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), user.user_id).await? {
        return Err(Status::Forbidden);
    }

    let mut tx = pool.inner()
        .begin()
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Delete from role_permissions first
    let role_permissions_count = sqlx::query!(
        r#"
        DELETE FROM role_permissions 
        WHERE permission_id = $1
        "#,
        permission_data.id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Delete the permission itself
    let permission_result = sqlx::query!(
        r#"
        DELETE FROM permissions 
        WHERE id = $1
        RETURNING id, name, created_at
        "#,
        permission_data.id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    let permission = permission_result.ok_or(Status::NotFound)?;

    tx.commit()
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Log successful
    let log = CreateLog {
        user_id: Some(admin_user.user_id),
        action: "permission_deleted_successfully".to_string(),
        details: json!({
            "permission_id": permission_data.id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "id": permission.id,
        "name": permission.name,
        "created_at": permission.created_at,
        "deleted_associations": {
            "role_permissions": role_permissions_count.rows_affected()
        }
    })))
}