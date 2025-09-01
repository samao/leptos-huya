#![allow(dead_code)]
#![allow(unused)]

use database::establish_connection;
use database::vod_page::{VodPage, get_cate_vods, get_hot_vods};
use diesel::SqliteConnection;
use tokio::fs::read_to_string;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = &mut establish_connection();
    // save(conn).await?;
    info!("读取热门视频===");
    get_hot_vods(conn)?;
    info!("读取分类视频===");
    get_cate_vods(conn)?;
    Ok(())
}

async fn save(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let json_str = read_to_string("database/data/vods-api.json").await?;
    let all: VodPage = serde_json::from_str(&json_str).unwrap();
    all.save(conn)?;
    Ok(())
}
