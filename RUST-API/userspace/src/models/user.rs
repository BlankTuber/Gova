use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,

    #[serde(skip_serializing)]
    password_hash: String,

    pub role: UserRole,

    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,

    pub status: UserStatus,
    pub profile: Option<Profile>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserRole {
    Admin,
    Moderator,
    User
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserStatus {
    Active,
    Suspended,
    Disabled,
    Deletion
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Profile {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
}
