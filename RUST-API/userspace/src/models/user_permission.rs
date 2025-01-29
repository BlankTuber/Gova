use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::user::User;
use super::permission::Permission;
use crate::models::schema::user_permissions;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_permissions)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Permission, foreign_key = permission_id))]
pub struct UserPermission {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub permission_id: i32,
    pub value: Option<bool>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_permissions)]
pub struct NewUserPermission {
    pub user_id: uuid::Uuid,
    pub permission_id: i32,
    pub value: Option<bool>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_permissions)]
pub struct UpdateUserPermission {
    pub value: Option<bool>,  // TRUE = allow, FALSE = deny, NULL = inherit
}