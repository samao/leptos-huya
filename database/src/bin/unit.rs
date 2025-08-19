use anyhow::Ok;
use clap::{Parser, Subcommand, arg};
use database::{dilevery_hot, get_all_hot_by_cate_id, get_cates, get_rooms};
use tracing::info;

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    Room {
        #[arg(short, long, value_delimiter = ',', help = "房间列表id")]
        ids: Vec<i32>,
    },
    Cate {
        #[arg(short, long, value_delimiter = ',', help = "房间列表id")]
        ids: Vec<i32>,
    },
    GetHot {
        #[arg(short, long, help = "分类id")]
        id: i32,
    },
    Trans {
        #[arg(short, long, help = "源id")]
        from: i32,
        #[arg(short, long, help = "目标id")]
        to: i32,
        #[arg(short, long, help = "转移数量")]
        amount: u8,
    },
}

fn main() -> anyhow::Result<()> {
    use database::establish_connection;
    let conn = &mut establish_connection();
    match Args::parse().cmd {
        Some(Command::Room { ids }) => {
            info!("要查看的Room IDS: {:?}", ids);
            get_rooms(conn, ids)?;
        }
        Some(Command::Cate { ids }) => {
            info!("要查看的CATE ID: {:?}", ids);
            get_cates(conn, ids.clone(), None)?;
        }
        Some(Command::GetHot { id }) => {
            info!("要查看的CATE ID: {}", id);
            get_all_hot_by_cate_id(conn, id)?;
        }
        Some(Command::Trans { from, to, amount }) => {
            dilevery_hot(conn, from, to, amount)?;
        }
        _ => {}
    }
    Ok(())
}
