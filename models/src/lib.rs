use chrono::Local;
use hex::ToHex;
use serde::{Deserialize, Serialize};
use sha2::Digest;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct User {
    pub id: i32,
    pub user_name: String,
    pub avatar: String,
    pub phone: String,
    #[serde(skip_serializing)]
    pub password: String,
}

impl User {
    pub fn compute_token(u: &User, time: i64) -> Result<String, serde_json::Error> {
        let json_str = serde_json::to_string(u)?;
        let mut hasher = sha2::Sha512::new();
        hasher.update(&json_str.as_bytes());
        hasher.update(time.to_be_bytes());
        let json_str = hasher.finalize();
        let token = json_str.encode_hex::<String>();
        Ok(token)
    }

    pub fn compute_token_with_now(u: &User) -> Result<String, serde_json::Error> {
        let time = Local::now();
        User::compute_token(u, time.timestamp())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenUser {
    #[serde(flatten)]
    pub user: User,
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::User;
    use chrono::DateTime;
    #[test]
    fn test_token() {
        let time = DateTime::parse_from_str("2025-11-01 00:00 +0800", "%Y-%m-%d %H:%M %z")
            .unwrap()
            .timestamp();

        let u = User::compute_token(
            &User {
                user_name: "王二小".to_string(),
                id: 0,
                avatar: "pic".to_string(),
                phone: "123".to_string(),
                password: "OOOO".to_string(),
            },
            time,
        )
        .unwrap();
        assert_eq!(
            u,
            "1a5e9a0327ee3a2e47f026b533f772aacda204633d86e91d5fd0aadd122f89a48a496e1597b29f847a68ada4cf882afcb5bd1f22393028c3e61801d89b63221d"
        );
    }
}
