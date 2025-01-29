use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::schema::permissions;

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = permissions)]
pub struct Permission {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = permissions)]
pub struct NewPermission {
    pub name: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = permissions)]
pub struct UpdatePermission {
    pub name: Option<String>,
    pub description: Option<String>,
}