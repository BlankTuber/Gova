use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::models::log::CreateLog;
use crate::utils::auth::is_admin;
use crate::models::user_roles::AssignRole;
use crate::models::user::DeleteUser;
use crate::utils::logger::{log_action, LogAction, LogBuilder};

#[post("/role/user", format="json", data="<user_data>")]
pub async fn assign_role_to_user(
    pool: &State<PgPool>,
    user_data: Json<AssignRole>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let user = user_data.into_inner();

    // Proper validation error handling
    if let Err(_) = user.validate() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
        INSERT INTO user_roles (user_id, role_id)
        VALUES ($1, $2)
        "#, user.user_id, user.role_id
    )
    .execute(pool.inner())
    .await;

    // Log successful
    let log = CreateLog {
        user_id: Some(admin_user.user_id),
        action: "user_added_to_role_successfull".to_string(),
        details: json!({
            "user_id": user.user_id,
            "role_id": user.role_id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    match result {
        Ok(_) => Ok(Json(json!({ "message": "User successfully assigned to role!" }))),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(Status::Conflict),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[delete("/role/user", format="json", data="<user_data>")]
pub async fn remove_user_from_role(
    pool: &State<PgPool>,
    user_data: Json<AssignRole>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let user = user_data.into_inner();

    // Proper validation error handling
    if let Err(_) = user.validate() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
        DELETE FROM user_roles
        WHERE user_id = $1
        AND role_id = $2
        "#, user.user_id, user.role_id
    )
    .execute(pool.inner())
    .await;

    // Log successful
    let log = CreateLog {
        user_id: Some(admin_user.user_id),
        action: "user_removed_from_role_successfull".to_string(),
        details: json!({
            "user_id": user.user_id,
            "role_id": user.role_id,
        }),
    };
    let _ = log_action(pool.inner(), &log).await;

    match result {
        Ok(_) => Ok(Json(json!({ "message": "User successfully removed from role!" }))),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/users")]
pub async fn get_all_users(
    pool: &State<PgPool>,
    admin_user: AuthenticatedUser
) -> Result<Json<Value>, Status> {
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let users = sqlx::query!(
        r#"
        SELECT id, username, email FROM users
        "#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Convert the query result into JSON
    let users_json: Vec<Value> = users.iter().map(|user| {
        json!({
            "id": user.id,
            "username": user.username,
            "email": user.email,
        })
    }).collect();

    // Create log entry for the successful users fetch
    let log = LogBuilder::new(LogAction::Read, "users")
        .with_user(admin_user.user_id)
        .with_additional_details(&json!({
            "total_users_fetched": users.len(),
            "fetched_by": admin_user.user_id,
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    Ok(Json(json!({
        "message": "Found users!",
        "users": users_json
    })))
}

#[delete("/user", format="json", data="<user_data>")]
pub async fn delete_user(
    admin_user: AuthenticatedUser,
    pool: &State<PgPool>,
    user_data: Json<DeleteUser>
) -> Result<Json<Value>, Status> {
    // Ensure the requesting user has admin privileges
    if !is_admin(pool.inner(), admin_user.user_id).await.map_err(|_| Status::InternalServerError)? {
        return Err(Status::Forbidden);
    }

    let user = user_data.into_inner();
    
    // Fetch user data before deletion for logging
    let user_to_delete = sqlx::query!(
        "SELECT email, username, created_at FROM users WHERE id = $1",
        user.id
    )
    .fetch_optional(pool.inner())
    .await
    .map_err(|_| Status::InternalServerError)?
    .ok_or(Status::NotFound)?;
    
    // Validate incoming request data
    if let Err(_) = user.validate() {
        return Err(Status::BadRequest);
    }

    // Start a transaction
    let mut tx = pool.inner().begin().await.map_err(|_| Status::InternalServerError)?;

    // Delete user_role first (foreign key relationship)
    let role_delete_result = sqlx::query!(
        r#"
        DELETE FROM user_roles WHERE user_id = $1
        "#,
        user.id
    )
    .execute(&mut *tx)
    .await;

    if let Err(_) = role_delete_result {
        // Rollback the transaction if role deletion fails
        let _ = tx.rollback().await;
        return Err(Status::InternalServerError);
    }

    // Delete user from the database
    let user_delete_result = sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1
        "#,
        user.id
    )
    .execute(&mut *tx)
    .await;

    // Log successful
    let log = LogBuilder::new(LogAction::Delete, "user")
        .with_user(admin_user.user_id)
        .with_resource_id(user.id.to_string())
        .with_previous_state(&json!({
            "email": user_to_delete.email,
            "username": user_to_delete.username,
            "created_at": user_to_delete.created_at,
        }))
        .map_err(|_| Status::InternalServerError)?
        .with_additional_details(&json!({
            "deleted_by": admin_user.user_id,
            "deletion_timestamp": chrono::Utc::now().to_rfc3339(),
        }))
        .map_err(|_| Status::InternalServerError)?
        .build();

    let _ = log_action(pool.inner(), &log).await;

    match user_delete_result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                let _ = tx.rollback().await;
                Err(Status::NotFound)
            } else {
                // Commit the transaction if both operations succeeded
                tx.commit().await.map_err(|_| Status::InternalServerError)?;
                Ok(Json(json!({ "message": "User and associated roles successfully deleted!" })))
            }
        },
        Err(_) => {
            let _ = tx.rollback().await;
            Err(Status::InternalServerError)
        }
    }
}