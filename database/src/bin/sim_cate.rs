use database::{establish_connection, models::SimCate, schema::sim_cate::dsl::*};
use diesel::prelude::*;
use regex::Regex;
use tokio::fs::{read_to_string, write};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let conn = &mut establish_connection();

    let reg = Regex::new(r"/([^/?]+)(?:\?.*)?$").unwrap();
    let file = read_to_string("app/public/imgs/cates/image_data.txt").await?;
    let mut lines = file.lines();
    while let Some(item) = lines.next()
        && let Some(title) = lines.next()
    {
        let filename = reg
            .captures(item)
            .and_then(|c| c.get(1))
            .map(|s| s.as_str().to_string())
            .unwrap();
        println!("file: {}, name: {}", filename, title);
        let file_byte = reqwest::get(item).await?.bytes().await?;
        write(format!("app/public/imgs/cates/{}", filename), file_byte).await?;
        let cate = diesel::insert_into(sim_cate)
            .values((src.eq(format!("/imgs/cates/{}", filename)), name.eq(title)))
            .returning(SimCate::as_returning())
            .get_result(conn)
            .unwrap();
        println!("{:?}", cate);
        lines.next();
    }
    Ok(())
}
