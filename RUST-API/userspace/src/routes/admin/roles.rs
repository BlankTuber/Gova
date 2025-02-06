use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::models::log::CreateLog;
use crate::models::role::CreateRole;
use crate::models::role::DeleteRole;
use crate::models::role_permissions::AssignPermission;
use crate::utils::auth::is_admin;
use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::utils::logger::log_action;

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

    // Log successful
    let log = CreateLog {
        user_id: Some(user.user_id),
        action: "role_created_successfully".to_string(),
        details: json!({
            "role_id": result.id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "id": result.id,
        "name": result.name,
        "created_at": result.created_at
    })))
}


#[delete("/role", format = "json", data = "<role_data>")]
pub async fn delete_role(
    pool: &State<PgPool>,
    user: AuthenticatedUser,
    role_data: Json<DeleteRole>,
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), user.user_id).await? {
        return Err(Status::Forbidden);
    }

    let mut tx = pool.inner()
        .begin()
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Delete from role_permissions and get count
    let role_permissions_count = sqlx::query!(
        r#"
        DELETE FROM role_permissions 
        WHERE role_id = $1
        "#,
        role_data.id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Delete from user_roles and get count
    let user_roles_count = sqlx::query!(
        r#"
        DELETE FROM user_roles 
        WHERE role_id = $1
        "#,
        role_data.id,
    )
    .execute(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Delete the role itself
    let role_result = sqlx::query!(
        r#"
        DELETE FROM roles 
        WHERE id = $1
        RETURNING id, name, created_at
        "#,
        role_data.id,
    )
    .fetch_optional(&mut *tx)
    .await
    .map_err(|_| Status::InternalServerError)?;

    let role = role_result.ok_or(Status::NotFound)?;

    tx.commit()
        .await
        .map_err(|_| Status::InternalServerError)?;

    // Log successful
    let log = CreateLog {
        user_id: Some(user.user_id),
        action: "role_deleted_successfully".to_string(),
        details: json!({
            "role_id": role_data.id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "id": role.id,
        "name": role.name,
        "created_at": role.created_at,
        "deleted_associations": {
            "role_permissions": role_permissions_count.rows_affected(),
            "user_roles": user_roles_count.rows_affected()
        }
    })))
}

#[post("/permission/role", format="json", data="<permission_data>")]
pub async fn assign_permission_to_role(
    pool: &State<PgPool>,
    permission_data: Json<AssignPermission>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let permission = permission_data.into_inner();

    // Validate input
    if let Err(_) = permission.validate() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO role_permissions (role_id, permission_id)
        VALUES ($1, $2)
        "#, permission.role_id, permission.permission_id
    )
    .execute(pool.inner())
    .await;

    if result.is_err() {
        return Err(Status::InternalServerError);
    }

    // Log successful
    let log = CreateLog {
        user_id: Some(admin_user.user_id),
        action: "permission_added_to_role_successfully".to_string(),
        details: json!({
            "role_id": permission.role_id,
            "permission_id": permission.permission_id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({ "message": "Permission successfully assigned to role!" })))
}



#[get("/roles")]
pub async fn get_all_roles(
    pool: &State<PgPool>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let roles = sqlx::query!(
        r#"
        SELECT id, name FROM roles
        "#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Convert the query result into JSON
    let roles_json: Vec<Value> = roles.iter().map(|role| { // Changed variable name from `roles` to `role`
        json!({
            "id": role.id,
            "name": role.name,
        })
    }).collect();

    Ok(Json(json!({
        "message": "Found roles!",
        "roles": roles_json
    })))
}