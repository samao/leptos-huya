use crate::models::Tag;
use crate::schema::tags::dsl::*;
use diesel::prelude::*;
use tracing::info;

pub fn create(conn: &mut SqliteConnection, input_tag: String) -> anyhow::Result<()> {
    let tag = diesel::insert_into(tags)
        .values(title.eq(input_tag))
        .returning(Tag::as_returning())
        .get_result(conn)?;
    info!("insert a Tag: {:?}", tag);
    Ok(())
}

pub fn update(conn: &mut SqliteConnection, input_id: i32, input_tag: String) -> anyhow::Result<()> {
    let tag = diesel::update(tags.filter(id.eq(input_id)))
        .set(title.eq(input_tag))
        .returning(Tag::as_returning())
        .get_result(conn)?;
    info!("update a Tag: {:?}", tag);
    Ok(())
}

pub fn read(conn: &mut SqliteConnection, input_id: Option<i32>) -> anyhow::Result<()> {
    let mut query = tags.into_boxed();
    if let Some(input_id) = input_id {
        query = query.filter(id.eq(input_id));
    }
    let tag_list: Vec<Tag> = query
        .select(Tag::as_select())
        .order_by(id.asc())
        .get_results(conn)?;
    info!("query tag list: {:?}", tag_list);
    Ok(())
}

pub fn delete(conn: &mut SqliteConnection, input_id: i32) -> anyhow::Result<()> {
    let tag_id = diesel::delete(tags.filter(id.eq(input_id)))
        .returning(Tag::as_returning())
        .get_result(conn)?;
    info!("remove tag: {:?}", tag_id);
    Ok(())
}
