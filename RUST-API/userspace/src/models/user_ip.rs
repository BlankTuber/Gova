use diesel::prelude::*;

use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;
use super::user::User;
use crate::models::schema::user_ips;

use diesel::expression::AsExpression;
#[derive(Debug, AsExpression, Serialize, Deserialize)]
#[diesel(sql_type = diesel::sql_types::Inet)]
pub struct IpAddressString(pub String);



#[derive(Queryable, Insertable, Identifiable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct UserIP {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub ip_address: IpAddressString,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
pub struct NewUserIP {
    pub user_id: uuid::Uuid,
    pub ip_address: IpAddressString,
}

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_ips)]
pub struct UpdateUserIP {
    pub ip_address: Option<IpAddressString>,
}