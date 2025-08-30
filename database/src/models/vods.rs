use crate::models::{User, VodCate};
use crate::schema::vods;
use diesel::prelude::*;
use serde::Serialize;

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
#[diesel(belongs_to(VodCate))]
#[diesel(table_name = vods)]
pub struct Vod {
    pub id: i32,
    pub img_url: String,
    pub duration: i32,
    pub user_id: i32,
    pub title: String,
    pub vod_cate_id: i32,
}
