use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::user::User; // Import User model

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_profiles)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserProfile {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_profiles)]
pub struct NewUserProfile {
    pub user_id: uuid::Uuid,
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_profiles)]
pub struct UpdateUserProfile {
    pub display_name: Option<String>,
    pub profile_picture: Option<String>,
    pub bio: Option<String>,
}