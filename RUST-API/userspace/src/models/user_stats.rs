use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use super::user::User;

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_stats)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserStats {
    pub user_id: uuid::Uuid, 
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
    pub last_activity: Option<NaiveDateTime>,
    pub last_login_ip: Option<std::net::IpAddr>,
}
