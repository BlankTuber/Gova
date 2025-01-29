use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use super::user::User;
use crate::models::schema::user_stats;

use diesel::expression::AsExpression;
#[derive(Debug, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = diesel::sql_types::Inet)]
pub struct IpAddressString(pub String);

#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(primary_key(user_id))]
#[diesel(table_name = user_stats)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserStats {
    pub user_id: uuid::Uuid, 
    pub created_at: NaiveDateTime,
    pub last_login: Option<NaiveDateTime>,
    pub last_activity: Option<NaiveDateTime>,
    pub last_login_ip: Option<IpAddressString>,
}
