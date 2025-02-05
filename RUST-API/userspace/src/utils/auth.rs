use sqlx::PgPool;
use uuid::Uuid;
use rocket::http::Status;

/// Check if a user is an admin
pub async fn is_admin(pool: &PgPool, user_id: Uuid) -> Result<bool, Status> {
    let is_admin = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1 FROM user_roles ur
            JOIN roles r ON ur.role_id = r.id
            WHERE ur.user_id = $1 AND r.name = 'Admin'
        )
        "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| Status::InternalServerError)?;

    Ok(is_admin.unwrap_or(false))
}
