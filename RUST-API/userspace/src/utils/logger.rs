use anyhow::{Context, Result};
use chrono::Utc;
use crate::models::log::{Log, CreateLog};
use uuid::Uuid;

pub async fn log_action(
    pool: &sqlx::PgPool,
    create_log: &CreateLog,
) -> Result<bool> {
    let log = Log {
        id: Uuid::new_v4(),
        user_id: create_log.user_id,
        action: create_log.action.clone(),
        details: create_log.details.clone(),
        created_at: Utc::now(),
    };

    // First validate the create_log
    create_log.validate()
        .context("Failed to validate log entry")?;

    let result = sqlx::query!(
        r#"
        INSERT INTO logs (id, user_id, action, details, created_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        log.id,
        log.user_id,
        log.action,
        log.details,
        log.created_at,
    )
    .execute(pool)
    .await
    .context("Failed to insert log entry into database")?;

    Ok(result.rows_affected() > 0)
}