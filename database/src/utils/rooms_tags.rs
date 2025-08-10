use crate::models::RoomTags;
use crate::schema::rooms_tags::{self, dsl::*};
use diesel::prelude::*;
use tracing::info;

pub fn create(conn: &mut SqliteConnection, rid: i32, tid: i32) -> anyhow::Result<()> {
    #[derive(Insertable)]
    #[diesel(table_name = rooms_tags)]
    struct InsertTag {
        room_id: i32,
        tag_id: i32,
    }

    let room_tag = diesel::insert_into(rooms_tags)
        .values(&InsertTag {
            room_id: rid,
            tag_id: tid,
        })
        .returning(RoomTags::as_returning())
        .get_result(conn)?;
    info!("create room tags: {:?}", room_tag);
    Ok(())
}

pub fn read(conn: &mut SqliteConnection, rid: Option<i32>, tid: Option<i32>) -> anyhow::Result<()> {
    let mut query = rooms_tags.into_boxed();
    if let Some(rid) = rid {
        query = query.filter(room_id.eq(rid));
    }
    if let Some(tid) = tid {
        query = query.filter(tag_id.eq(tid));
    }
    let room_tags = query.select(RoomTags::as_select()).get_results(conn)?;
    info!("read room tags: {:?}", room_tags);
    Ok(())
}
pub fn delete(
    conn: &mut SqliteConnection,
    rid: Option<i32>,
    tid: Option<i32>,
) -> anyhow::Result<()> {
    let mut query = diesel::delete(rooms_tags).into_boxed();
    if let Some(rid) = rid {
        query = query.filter(room_id.eq(rid));
    }
    if let Some(tid) = tid {
        query = query.filter(tag_id.eq(tid));
    }
    let room_tags = query
        .returning(RoomTags::as_returning())
        .get_results(conn)?;
    info!("get room tags: {:?}", room_tags);

    Ok(())
}
