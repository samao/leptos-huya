use serde::{Deserialize, Serialize};

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
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SimCate {
    pub id: i32,
    pub src: String,
    pub name: String,
}
