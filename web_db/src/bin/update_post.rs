use diesel::prelude::*;
use std::env::args;
use web_db::{models::Post, *};

fn main() {
    use self::schema::posts::dsl::*;

    let update_id = args()
        .nth(1)
        .expect("A post id required")
        .parse::<i32>()
        .expect("INVALID ID");

    let msg = args().nth(2).expect("Need some Body");

    let conn = &mut establish_connection();

    let result_post = diesel::update(posts)
        .filter(id.eq(update_id))
        .set((published.eq(true), body.eq(msg)))
        .returning(Post::as_returning())
        .get_result(conn)
        .unwrap();

    println!(
        "{}. title: {}, status: {}",
        result_post.id, result_post.title, result_post.published
    );
}
