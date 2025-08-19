use diesel::prelude::*;
use serde::Serialize;

use crate::models::{Cate, User};
use crate::schema::rooms;

#[derive(
    Queryable,
    Identifiable,
    Associations,
    Selectable,
    QueryableByName,
    Debug,
    PartialEq,
    Serialize,
    Clone,
    Hash,
    Eq,
)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Cate))]
#[diesel(table_name = rooms)]
pub struct Room {
    pub id: i32,
    pub title: String,
    pub is_live: bool,
    pub img_url: String,
    pub hot: i32,
    pub user_id: i32,
    pub cate_id: i32,
}
