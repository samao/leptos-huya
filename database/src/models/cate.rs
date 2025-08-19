use diesel::prelude::*;
use serde::Serialize;

// ------------------
use crate::schema::cates;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone, Hash, Eq)]
#[diesel(table_name = cates)]
pub struct Cate {
    pub id: i32,
    pub icon_url: String,
    pub img_url: String,
    pub cate_name: String,
    pub live_total: i32,
}
