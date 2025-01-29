use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::user::User;
use super::role::Role;
use crate::models::schema::user_roles;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_roles)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Role, foreign_key = role_id))]
pub struct UserRole {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub role_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_roles)]
pub struct NewUserRole {
    pub user_id: uuid::Uuid,
    pub role_id: i32,
}