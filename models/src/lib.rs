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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VodUser {
    pub name: String,
    pub avatar: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullVod {
    pub id: i32,
    pub img_url: String,
    pub duration: i32,
    pub user: VodUser,
    pub title: String,
    pub vod_cate_id: i32,
    pub hots: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Vod {
    pub title: String,
    pub img_url: String,
    pub hots: i32,
    pub duration: String,
    pub owner: VodUser,
}

impl Vod {
    pub fn duration_as_sec(&self) -> i32 {
        let mut index = 0;
        let times = self
            .duration
            .split(":")
            .map(|item| item.parse::<i32>().unwrap_or_default())
            .collect::<Vec<_>>()
            .iter()
            .rfold(0, |total, acc| {
                let next_total = total + acc * 60i32.pow(index);
                index += 1;
                next_total
            });
        times
    }
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VodSet {
    pub title: String,
    pub cover: String,
    pub tags: Vec<String>,
    pub list: Vec<Vod>,
    pub rank: Vec<Vod>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Site {
    pub title: String,
    pub list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Banner {
    pub title: String,
    pub cover: String,
}

#[derive(Debug, Deserialize)]
pub struct VodPage {
    pub banners: Vec<Banner>,
    pub sites: Vec<Site>,
    pub aggregations: Vec<VodSet>,
    pub recommends: Vec<Vod>,
}
