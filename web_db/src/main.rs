use self::models::*;
use diesel::prelude::*;
use web_db::*;

#[tokio::main]
async fn main() {
    use self::schema::posts::dsl::*;

    let connection = &mut establish_connection();

    // read
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .select(Post::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts\n", results.len());

    for post in results {
        println!(
            "{}.title: {}, status: {}",
            post.id, post.title, post.published
        );
        println!("------------\n");
        println!("{}\n", post.body);
    }
}
