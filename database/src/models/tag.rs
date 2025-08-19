use diesel::prelude::*;
use serde::Serialize;

use crate::schema::tags;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone)]
#[diesel(table_name = tags)]
pub struct Tag {
    pub id: i32,
    pub title: String,
}
