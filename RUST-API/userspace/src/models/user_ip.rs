use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use super::user::User;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserIP {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub ip_address: std::net::IpAddr,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
pub struct NewUserIP {
    pub user_id: uuid::Uuid,
    pub ip_address: std::net::IpAddr,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
pub struct UpdateUserIP {
    pub ip_address: Option<std::net::IpAddr>,
}