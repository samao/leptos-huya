use crate::models::User;
use crate::schema::tokens;
use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, PartialEq, Clone)]
#[diesel(belongs_to(User))]
#[diesel(table_name = tokens)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub create_at: NaiveDateTime,
    pub expired_at: NaiveDateTime,
}
