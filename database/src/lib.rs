use diesel::{prelude::*, r2d2::ConnectionManager};
use dotenvy::dotenv;
use r2d2::{Pool, PooledConnection};
use std::{
    env,
    fs::{File, create_dir_all},
    path::Path,
    sync::LazyLock,
};
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, warn};
use tracing_subscriber::FmtSubscriber;

pub mod models;
pub mod schema;

mod utils;
pub use utils::*;

static DB_POOL: LazyLock<Pool<ConnectionManager<SqliteConnection>>> = LazyLock::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL muse be set");
    info!("database url: {}", database_url);
    if !Path::new(&database_url).exists() {
        if database_url.starts_with("sqlite:") {
            warn!("\n1.can not create db file start with \"sqlite:\"\n2.directly use local path");
        } else {
            let db_path = Path::new(&database_url);
            if let Some(folder) = db_path.parent() {
                if let Err(err) = create_dir_all(folder) {
                    warn!("create folder fail: {:?}", err);
                }
            }
            if let Err(err) = File::create(&database_url) {
                warn!("\ncreate db file err: {:?}", err);
            } else {
                info!("\n1. run \"diesel migration run\" create all tables");
                panic!("2. try command again!");
            }
        }
    }
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect("Could not build connection pool!");
    pool
});

pub fn establish_connection() -> PooledConnection<ConnectionManager<SqliteConnection>> {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::DEBUG)
        .without_time()
        .finish();
    set_global_default(subscriber).ok();

    DB_POOL
        .get()
        .unwrap_or_else(|_| panic!("Error occur when get a connection from pool"))
}
