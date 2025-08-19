use crate::models::{Room, Tag};
use crate::schema::rooms_tags;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Identifiable, Selectable, Queryable, Associations, Serialize, Debug, PartialEq, Clone)]
#[diesel(belongs_to(Room))]
#[diesel(belongs_to(Tag))]
#[diesel(table_name = rooms_tags)]
#[diesel(primary_key(room_id, tag_id))]
pub struct RoomTags {
    pub room_id: i32,
    pub tag_id: i32,
}
