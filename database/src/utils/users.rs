use crate::models::User as DbUser;
use crate::schema::users::dsl::*;
use crate::utils::tokens;
use anyhow::anyhow;
use diesel::prelude::*;
use hex::ToHex;
use models::User as ModelUser;
use tracing::info;

pub fn login_by_id_or_phone(
    conn: &mut SqliteConnection,
    id_or_phone: i32,
    pwd: String,
) -> anyhow::Result<()> {
    let user = users
        .filter(
            id.eq(id_or_phone)
                .or(phone.eq(id_or_phone.to_string()))
                .and(password.eq(pwd)),
        )
        .select(DbUser::as_select())
        .get_result(conn)
        .map_err(|e| anyhow!(format!("用户名/密码错误: {:?}", e.to_string())))?;
    info!("登录用户: {:?}", user);
    Ok(())
}

pub fn create(
    conn: &mut SqliteConnection,
    input_name: String,
    input_head: Option<String>,
    input_phone: String,
    input_passowrd: Option<String>,
) -> anyhow::Result<DbUser> {
    info!("run create user");
    let user = diesel::insert_into(users)
        .values((
            user_name.eq(input_name),
            input_head.map(|head_url| avatar.eq(head_url)),
            phone.eq(input_phone),
            input_passowrd.map(|e| password.eq(e.encode_hex::<String>())),
        ))
        .returning(DbUser::as_returning())
        .get_result(conn)?;
    info!("insert a User: {:?}", user);
    Ok(user)
}

pub fn update(
    conn: &mut SqliteConnection,
    input_id: i32,
    input_name: Option<String>,
    input_head: Option<String>,
    input_phone: Option<String>,
    input_password: Option<String>,
) -> anyhow::Result<()> {
    let user = diesel::update(users.filter(id.eq(input_id)))
        .set((
            input_name.map(|uname| user_name.eq(uname)),
            input_head.map(|head_url| avatar.eq(head_url)),
            input_phone.map(|ph| phone.eq(ph)),
            input_password.map(|e| password.eq(e.encode_hex::<String>())),
        ))
        .returning(DbUser::as_returning())
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
        .select(DbUser::as_select())
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

pub fn register(
    conn: &mut SqliteConnection,
    input_phone: String,
    input_passowrd: Option<String>,
) -> Result<ModelUser, String> {
    let name = mock_rust::mock::cname();

    let user = create(conn, name, None, input_phone, input_passowrd)
        .map_err(|er| format!("register failed: {:?}", er.to_string()))?;
    let user: ModelUser = user.try_into()?;

    tokens::create(conn, user.id, user.token.clone())
        .map_err(|e| format!("{:?}", e.to_string()))?;
    Ok(user)
}

pub fn login(
    conn: &mut SqliteConnection,
    input_id: String,
    pwd: String,
) -> Result<ModelUser, String> {
    let input_id = input_id
        .parse::<i32>()
        .map_err(|_| format!("输入的id参数有误"))?;
    let user = users
        .into_boxed()
        .filter(id.eq(input_id))
        //.filter(password.eq(pwd.encode_hex::<String>()))
        .select(DbUser::as_returning())
        .first(conn)
        .map_err(|_| format!("login error no user found!"))?;
    if user.password != pwd.encode_hex::<String>() {
        return Err(format!("密码错误"));
    }
    let user: ModelUser = user.try_into()?;
    tokens::create(conn, user.id, user.token.clone())
        .map_err(|e| format!("{:?}", e.to_string()))?;
    Ok(user)
}

pub fn phone_login(
    conn: &mut SqliteConnection,
    input_phone: String,
    _sms: String,
) -> Result<ModelUser, String> {
    let user = users
        .into_boxed()
        .filter(phone.eq(input_phone))
        .select(DbUser::as_returning())
        .first(conn)
        .map_err(|_| format!("mobile login failed!"))?;
    let user: ModelUser = user.try_into()?;
    tokens::create(conn, user.id, user.token.clone())
        .map_err(|e| format!("{:?}", e.to_string()))?;
    Ok(user)
}
