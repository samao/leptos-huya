use crate::models::Cate;
use crate::schema::cates;
use crate::schema::cates::dsl::*;
use diesel::SqliteConnection;
use diesel::prelude::*;

pub fn update(
    conn: &mut SqliteConnection,
    input_id: i32,
    icon: Option<String>,
    big_icon: Option<String>,
    name: Option<String>,
    total: Option<i32>,
) -> anyhow::Result<()> {
    #[derive(Debug, AsChangeset)]
    #[diesel(table_name = cates)]
    struct UpdateCate {
        icon_url: Option<String>,
        img_url: Option<String>,
        cate_name: Option<String>,
        live_total: Option<i32>,
    }

    let cate = diesel::update(cates.filter(id.eq(input_id)))
        .set((&UpdateCate {
            icon_url: icon,
            img_url: big_icon,
            cate_name: name,
            live_total: total,
        },))
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("update a cate: {:?}", cate);
    Ok(())
}

pub fn read(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let all_cates = cates.select(Cate::as_returning()).load(conn)?;
    println!("all cate:\n{:#?}", all_cates);
    Ok(())
}

pub fn create(
    conn: &mut SqliteConnection,
    icon: Option<String>,
    big_icon: Option<String>,
    name: String,
    total: Option<i32>,
) -> anyhow::Result<()> {
    #[derive(Debug, Insertable)]
    #[diesel(table_name = cates)]
    struct InsertCate {
        icon_url: Option<String>,
        img_url: Option<String>,
        cate_name: String,
        live_total: Option<i32>,
    }

    let cate = diesel::insert_into(cates)
        .values(&InsertCate {
            cate_name: name,
            icon_url: icon,
            img_url: big_icon,
            live_total: total,
        })
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("create a cate: {:?}", cate);
    Ok(())
}

pub fn delete(conn: &mut SqliteConnection, cid: i32) -> anyhow::Result<()> {
    let cate = diesel::delete(cates)
        .filter(id.eq(cid))
        .returning(Cate::as_returning())
        .get_result(conn)?;
    println!("del a cate: {:?}", cate);
    Ok(())
}
