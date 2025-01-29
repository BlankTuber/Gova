use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::models::schema::roles;

#[derive(Queryable, Insertable, Identifiable, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct Role {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct NewRole {
    pub name: String,
    pub description: Option<String>,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = roles)]
pub struct UpdateRole {
    pub name: Option<String>,
    pub description: Option<String>,
}