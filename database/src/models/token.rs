use crate::models::User;
use crate::schema::tokens;
use diesel::prelude::*;
use serde::Serialize;
use std::time::SystemTime;

#[derive(Identifiable, Selectable, Queryable, Associations, Serialize, Debug, PartialEq, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub create_at: SystemTime,
}
