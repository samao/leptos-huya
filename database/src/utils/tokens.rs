use crate::models::{Token, User as DbUser};
use crate::schema::{tokens::dsl::*, users};
use anyhow::anyhow;
use chrono::{Duration, Local};
use diesel::prelude::*;
use models::User as ModelUser;
use tracing::info;

pub fn read(conn: &mut SqliteConnection, uid: Option<i32>) -> anyhow::Result<()> {
    let mut query = tokens.into_boxed();
    if let Some(uid) = uid {
        query = query.filter(user_id.eq(uid));
    }
    let user_tokens = query.select(Token::as_select()).get_results(conn)?;
    info!("read token: {:?}", user_tokens);
    Ok(())
}

pub fn get_user(conn: &mut SqliteConnection, input_token: String) -> anyhow::Result<ModelUser> {
    let user = conn.transaction(|conn| {
        let user_token = tokens
            .filter(token.eq(input_token.clone()))
            .select(Token::as_returning())
            .first(conn)?;
        info!("找到了token: {:?}", user_token);
        if user_token.expired_at < Local::now().naive_local() {
            info!("登录信息已过期");
            delete(conn, input_token)?;
            return Err(anyhow!("expire token".to_string()));
        }
        let user = tokens
            .inner_join(users::table.on(user_id.eq(users::id)))
            .select(DbUser::as_returning())
            .get_result(conn)?;
        Ok(user)
    })?;
    let user = user
        .try_into()
        .map_err(|_| anyhow!("user 转换失败".to_string()))?;
    info!("登录的用户：{:?}", user);
    Ok(user)
}

pub fn create(conn: &mut SqliteConnection, uid: i32, input_token: String) -> anyhow::Result<()> {
    let expired = Local::now().naive_local() + Duration::days(30);

    let user_token = diesel::insert_into(tokens)
        .values((
            token.eq(input_token),
            user_id.eq(uid),
            expired_at.eq(expired),
        ))
        .returning(Token::as_returning())
        .get_results(conn)?;
    info!("create token: {:?}", user_token);
    Ok(())
}

pub fn delete(conn: &mut SqliteConnection, input_token: String) -> anyhow::Result<()> {
    let user_token = diesel::delete(tokens)
        .filter(token.eq(input_token))
        .returning(Token::as_returning())
        .get_result(conn)?;
    info!("delete token: {:?}", user_token);
    Ok(())
}

pub fn clear_user(conn: &mut SqliteConnection, uid: i32) -> anyhow::Result<()> {
    diesel::delete(tokens)
        .filter(user_id.eq(uid))
        .execute(conn)?;
    Ok(())
}
