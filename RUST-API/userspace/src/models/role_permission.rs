use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use super::role::Role;
use super::permission::Permission;
use crate::models::schema::role_permissions;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = role_permissions)]
#[diesel(belongs_to(Role, foreign_key = role_id))]
#[diesel(belongs_to(Permission, foreign_key = permission_id))]
pub struct RolePermission {
    pub id: i32,
    pub role_id: i32,
    pub permission_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = role_permissions)]
pub struct NewRolePermission {
    pub role_id: i32,
    pub permission_id: i32,
}