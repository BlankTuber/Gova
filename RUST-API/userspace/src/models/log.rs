use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;
use rocket::serde::json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow, Validate)]
pub struct Log {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub details: Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateLog {
    pub user_id: Option<Uuid>,
    
    #[validate(length(min = 3, max = 255))]
    pub action: String,

    pub details: Value,
}
