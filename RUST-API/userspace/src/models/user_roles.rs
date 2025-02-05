use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}
