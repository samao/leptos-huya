use diesel::prelude::*;
use serde::Serialize;

// ------------------
use crate::schema::{cates, rooms, rooms_tags, tags, users};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone, Hash, Eq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub avatar: String,
}

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone, Hash, Eq)]
#[diesel(table_name = cates)]
pub struct Cate {
    pub id: i32,
    pub icon_url: String,
    pub img_url: String,
    pub cate_name: String,
    pub live_total: i32,
}

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

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, PartialEq, Clone)]
#[diesel(belongs_to(Room))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = rooms_tags)]
#[diesel(primary_key(room_id, tag_id))]
pub struct RoomTags {
    pub room_id: i32,
    pub tag_id: i32,
}

// ---------------
