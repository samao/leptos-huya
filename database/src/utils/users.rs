use crate::models::User;
use crate::schema::users::dsl::*;
use anyhow::anyhow;
use diesel::prelude::*;
use tracing::info;

pub fn create(
    conn: &mut SqliteConnection,
    input_name: String,
    input_head: Option<String>,
) -> anyhow::Result<()> {
    let user = diesel::insert_into(users)
        .values((
            user_name.eq(input_name),
            input_head.map(|head_url| avatar.eq(head_url)),
        ))
        .returning(User::as_returning())
        .get_result(conn)?;
    info!("insert a User: {:?}", user);
    Ok(())
}

pub fn update(
    conn: &mut SqliteConnection,
    input_id: i32,
    input_name: Option<String>,
    input_head: Option<String>,
) -> anyhow::Result<()> {
    let user = diesel::update(users.filter(id.eq(input_id)))
        .set((
            input_name.map(|uname| user_name.eq(uname)),
            input_head.map(|head_url| avatar.eq(head_url)),
        ))
        .returning(User::as_returning())
        .get_result(conn)?;
    info!("update a User: {:?}", user);
    Ok(())
}

pub fn read(conn: &mut SqliteConnection, input_id: Option<i32>) -> anyhow::Result<()> {
    let mut query = users.into_boxed();
    if let Some(input_id) = input_id {
        query = query.filter(id.eq(input_id));
    }
    let user = query
        .select(User::as_select())
        .order_by(id.asc())
        .get_results(conn)?;
    info!("read User(s): {:?}", user);
    Ok(())
}

pub fn delete(conn: &mut SqliteConnection, input_id: i32) -> anyhow::Result<()> {
    let count = diesel::delete(users.filter(id.eq(input_id))).execute(conn)?;
    if count == 0 {
        return Err(anyhow!("No user found with id {}", input_id));
    }
    info!("deleted {} User(s)", count);
    Ok(())
}
