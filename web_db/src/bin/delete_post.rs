use diesel::prelude::*;
use std::{env::args, time::Duration};
use web_db::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use self::schema::posts::dsl::*;

    println!("START");
    tokio::time::sleep(Duration::from_secs(3)).await;
    let target = args().nth(1).expect("Expected a target to match against");

    let pattern = format!("%{}%", target);

    let conn = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);

    println!("END..");
    Ok(())
}
