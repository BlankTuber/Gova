use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct RolePermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct AssignPermission {
    pub role_id: Uuid,
    pub permission_id: Uuid,
}
