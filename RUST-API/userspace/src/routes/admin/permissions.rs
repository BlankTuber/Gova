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
use crate::utils::logger::{log_action, LogAction, LogBuilder};

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
        user_id: Some(user.user_id),
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

    // Fetch permission data and associations before deletion
    let permission_info = sqlx::query!(
        r#"
        SELECT p.name, p.created_at,
               COUNT(DISTINCT rp.role_id) as role_count,
               ARRAY_AGG(DISTINCT r.name) FILTER (WHERE r.name IS NOT NULL) as role_names
        FROM permissions p
        LEFT JOIN role_permissions rp ON p.id = rp.permission_id
        LEFT JOIN roles r ON rp.role_id = r.id
        WHERE p.id = $1
        GROUP BY p.id, p.name, p.created_at
        "#,
        permission_data.id
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::NotFound)?;

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

    let log = LogBuilder::new(LogAction::Delete, "permission")
        .with_user(user.user_id)
        .with_resource_id(permission_data.id.to_string())
        .with_previous_state(&json!({
            "name": permission_info.name,
            "created_at": permission_info.created_at,
            "associations": {
                "roles_count": permission_info.role_count,
                "associated_roles": permission_info.role_names,
            }
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "deleted_by": user.user_id,
            "deletion_timestamp": chrono::Utc::now().to_rfc3339(),
            "cascade_deletions": {
                "role_permissions_removed": role_permissions_count.rows_affected()
            }
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    tx.commit()
        .await
        .map_err(|_| Status::InternalServerError)?;

    Ok(Json(json!({
        "id": permission.id,
        "name": permission.name,
        "created_at": permission.created_at,
        "deleted_associations": {
            "role_permissions": role_permissions_count.rows_affected()
        }
    })))
}