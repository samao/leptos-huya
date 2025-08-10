use self::models::{NewPost, Post};
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_TEST_URL").expect("DATABASE_URL muse be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

pub fn create_post(conn: &mut SqliteConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .returning(Post::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn get_post(pid: i32) -> Option<Post> {
    use schema::posts::dsl::*;
    let conn = &mut establish_connection();

    let post = posts
        .find(pid)
        .select(Post::as_select())
        .first(conn)
        .optional();

    match post {
        Ok(Some(post)) => Some(post),
        Ok(None) => None,
        Err(_) => None,
    }
}
