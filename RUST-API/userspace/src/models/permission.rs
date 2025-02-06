use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreatePermission {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeletePermission {
    pub id: Uuid,
}