use crate::models::User;
use crate::schema::users::dsl::*;
use anyhow::anyhow;
use diesel::prelude::*;
use hex::ToHex;
use tracing::info;

pub fn create(
    conn: &mut SqliteConnection,
    input_name: String,
    input_head: Option<String>,
    input_phone: String,
    input_passowrd: Option<String>,
) -> anyhow::Result<User> {
    info!("run create user");
    let user = diesel::insert_into(users)
        .values((
            user_name.eq(input_name),
            input_head.map(|head_url| avatar.eq(head_url)),
            phone.eq(input_phone),
            input_passowrd.map(|e| password.eq(e.encode_hex::<String>())),
        ))
        .returning(User::as_returning())
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

pub fn register(
    conn: &mut SqliteConnection,
    input_phone: String,
    input_passowrd: Option<String>,
) -> Result<User, String> {
    create(
        conn,
        "王老二".to_string(),
        None,
        input_phone,
        input_passowrd,
    )
    .map_err(|_| format!("register failed"))
}

pub fn login(conn: &mut SqliteConnection, input_id: String, pwd: String) -> Result<User, String> {
    let input_id = input_id
        .parse::<i32>()
        .map_err(|_| format!("输入的id参数有误"))?;
    users
        .into_boxed()
        .filter(id.eq(input_id))
        .filter(password.eq(pwd.encode_hex::<String>()))
        .select(User::as_returning())
        .first(conn)
        .map_err(|_| format!("login error maybe pwd wrong!"))
}
pub fn phone_login(
    conn: &mut SqliteConnection,
    input_phone: String,
    _sms: String,
) -> Result<User, String> {
    users
        .into_boxed()
        .filter(phone.eq(input_phone))
        .select(User::as_returning())
        .first(conn)
        .map_err(|_| format!("mobile login failed!"))
}

pub fn phone_login_tk(
    conn: &mut SqliteConnection,
    input_phone: String,
    _sms: String,
) -> Result<models::TokenUser, String> {
    let user = users
        .into_boxed()
        .filter(phone.eq(input_phone))
        .select(User::as_returning())
        .first(conn)
        .map_err(|_| format!("mobile login failed!"))?;
    user.try_into()
}

impl TryFrom<User> for models::TokenUser {
    type Error = String;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        let usr = models::User {
            id: value.id,
            user_name: value.user_name,
            avatar: value.avatar,
            phone: value.phone,
            password: value.password,
        };
        let token =
            models::User::compute_token_with_now(&usr).map_err(|_| format!("生成token失败"))?;
        Ok(models::TokenUser { user: usr, token })
    }
}
