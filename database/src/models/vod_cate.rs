use crate::schema::{sub_cate, vod_cate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable,
    Identifiable,
    Insertable,
    Deserialize,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Clone,
)]
#[diesel(table_name = vod_cate)]
pub struct VodCate {
    pub id: i32,
    pub cate_name: String,
    pub img_url: String,
}

#[derive(
    Queryable,
    Identifiable,
    Insertable,
    Associations,
    Deserialize,
    Selectable,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Clone,
)]
#[diesel(belongs_to(VodCate))]
#[diesel(table_name = sub_cate)]
pub struct SubCate {
    pub id: i32,
    pub vod_cate_id: i32,
    pub cate_name: String,
}
