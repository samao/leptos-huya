use chrono::Local;
use diesel::prelude::*;
use hex::ToHex;
use models::User as ModelUser;
use serde::Serialize;
use serde_json::json;
use sha2::Digest;

use crate::schema::users;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Serialize, Clone, Hash, Eq)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub avatar: String,
    pub phone: String,
    pub password: String,
}

impl User {
    fn computed_token(&self) -> Result<String, serde_json::Error> {
        let now = Local::now();
        let json = serde_json::to_string(&json!({
            "id": self.id,
            "pwd": self.password,
        }))?;
        let mut hasher = sha2::Sha512::new();

        hasher.update(&json.as_bytes());
        hasher.update(now.timestamp().to_be_bytes());

        let token = hasher.finalize().encode_hex::<String>();

        Ok(token)
    }
}

impl TryFrom<User> for ModelUser {
    type Error = String;
    fn try_from(value: User) -> Result<Self, Self::Error> {
        Ok(models::User {
            id: value.id,
            user_name: value.user_name.clone(),
            avatar: value.avatar.clone(),
            phone: value.phone.clone(),
            token: value
                .computed_token()
                .map_err(|err| format!("token generate failed {:?}", err.to_string()))?,
        })
    }
}
