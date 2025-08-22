use crate::schema::sim_cate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Queryable, Identifiable, Insertable, Deserialize, Selectable, Debug, PartialEq, Serialize, Clone,
)]
#[diesel(table_name = sim_cate)]
pub struct SimCate {
    pub id: i32,
    pub src: String,
    pub name: String,
}

impl From<SimCate> for models::SimCate {
    fn from(value: SimCate) -> Self {
        Self {
            id: value.id,
            src: value.src,
            name: value.name,
        }
    }
}
