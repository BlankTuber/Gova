use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;
use rocket::serde::json::Value;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub social_links: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateUserProfile {
    pub user_id: Uuid,
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>,
    #[validate(length(max = 10))]
    pub language: Option<String>,
    #[validate(length(max = 50))]
    pub timezone: Option<String>,
    pub social_links: Option<Value>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UpdateUserProfile {
    #[validate(length(max = 100))]
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub birth_date: Option<NaiveDate>,
    #[validate(length(max = 10))]
    pub language: Option<String>,
    #[validate(length(max = 50))]
    pub timezone: Option<String>,
    pub social_links: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeleteUserProfile {
    pub id: Uuid,
}