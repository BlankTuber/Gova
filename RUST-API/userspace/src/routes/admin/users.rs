use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::utils::auth::is_admin;
use crate::models::user_roles::AssignRole;
use crate::models::user::DeleteUser;

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

    match result {
        Ok(_) => Ok(Json(json!({ "message": "User successfully removed from role!" }))),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => Err(Status::Conflict),
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
    
    // Validate incoming request data
    if let Err(_) = user.validate() {
        return Err(Status::BadRequest);
    }

    // Delete user from the database
    let result = sqlx::query!(
        r#"
        DELETE FROM users WHERE id = $1
        "#,
        user.id
    )
    .execute(pool.inner())
    .await;

    match result {
        Ok(query_result) => {
            if query_result.rows_affected() == 0 {
                Err(Status::NotFound)
            } else {
                Ok(Json(json!({ "message": "User successfully deleted!" })))
            }
        },
        Err(_) => Err(Status::InternalServerError),
    }
}