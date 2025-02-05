use rocket::serde::json::{Json, json};
use rocket::State;
use rocket::http::Status;
use rocket::serde::json::Value;
use sqlx::PgPool;
use validator::Validate;

use crate::middleware::verify_jwt::AuthenticatedUser;
use crate::models::user::UpdateUser;

#[put("/username", format="json", data="<username_data>")]
pub async fn update_username(
    user: AuthenticatedUser, 
    pool: &State<PgPool>, 
    username_data: Json<UpdateUser>
) -> Result<Json<Value>, Status> {
    let username = username_data.into_inner();

    // Validate the incoming username data
    if username.validate().is_err() {
        return Err(Status::BadRequest);
    }

    let result = sqlx::query!(
        r#"
            UPDATE users
            SET username = $1
            WHERE id = $2
        "#,
        username.username, 
        user.user_id
    )
    .execute(pool.inner()) // Use execute instead of fetch_optional()
    .await
    .map_err(|_| Status::InternalServerError)?;

    // Check if any rows were updated
    if result.rows_affected() == 0 {
        return Err(Status::NotFound);
    }

    Ok(Json(json!({
        "message": "Username successfully updated!"
    })))
}
