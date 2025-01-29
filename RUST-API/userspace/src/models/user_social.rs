use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::user::User;
use crate::models::schema::user_socials;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_socials)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserSocial {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub platform: String,
    pub url: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_socials)]
pub struct NewUserSocial {
    pub user_id: uuid::Uuid,
    pub platform: String,
    pub url: String,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_socials)]
pub struct UpdateUserSocial {
    pub platform: Option<String>,
    pub url: Option<String>,
}