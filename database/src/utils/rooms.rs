use crate::models::Room;
use crate::schema::rooms::dsl::*;
use crate::schema::rooms::{self};
use diesel::SqliteConnection;
use diesel::prelude::*;

pub fn update(
    conn: &mut SqliteConnection,
    rid: i32,
    room_name: Option<String>,
    live: Option<bool>,
    image: Option<String>,
    hot_val: Option<i32>,
    uid: Option<i32>,
    cateid: Option<i32>,
) -> anyhow::Result<()> {
    #[derive(AsChangeset)]
    #[diesel(table_name = rooms)]
    pub struct RoomUpdate {
        pub title: Option<String>,
        pub is_live: Option<bool>,
        pub img_url: Option<String>,
        pub hot: Option<i32>,
        pub user_id: Option<i32>,
        pub cate_id: Option<i32>,
    }

    let room = diesel::update(rooms.filter(id.eq(rid)))
        .set(&RoomUpdate {
            title: room_name.clone(),
            is_live: live,
            img_url: image.clone(),
            hot: hot_val,
            user_id: uid,
            cate_id: cateid,
        })
        .returning(Room::as_returning())
        .get_result(conn)?;
    println!("update room: {:?}", room);
    Ok(())
}

pub fn delete(conn: &mut SqliteConnection, rid: i32) -> anyhow::Result<()> {
    let dels = diesel::delete(rooms)
        .filter(id.eq(rid))
        .returning(Room::as_returning())
        .get_result(conn)?;
    println!("delete room: {:?}", dels);
    Ok(())
}

pub fn read(conn: &mut SqliteConnection, rid: Option<i32>) -> anyhow::Result<()> {
    let mut query = rooms.into_boxed();
    if let Some(rid) = rid {
        query = query.filter(id.eq(rid));
    }
    let room = query.select(Room::as_returning()).get_results(conn)?;
    println!("read room: {:?}", room);
    Ok(())
}

pub fn create(
    conn: &mut SqliteConnection,
    name: String,
    live: bool,
    img: String,
    hot_val: i32,
    user: i32,
    cateid: i32,
) -> anyhow::Result<()> {
    let room = diesel::insert_into(rooms)
        .values((
            title.eq(name),
            is_live.eq(live),
            img_url.eq(img),
            hot.eq(hot_val),
            user_id.eq(user),
            cate_id.eq(cateid),
        ))
        .returning(Room::as_returning())
        .get_result(conn)?;

    println!("创建新房间: {:?}", room);

    Ok(())
}
