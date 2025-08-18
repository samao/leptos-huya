use diesel::prelude::*;
use dotenvy::dotenv;
use std::{
    env,
    fs::{File, create_dir_all},
    path::Path,
};
use tracing::{info, level_filters::LevelFilter, subscriber::set_global_default, warn};
use tracing_subscriber::FmtSubscriber;

pub mod models;
pub mod schema;

mod utils;
pub use utils::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::DEBUG)
        .finish();
    set_global_default(subscriber).ok();

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

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
