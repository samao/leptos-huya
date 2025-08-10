use clap::{ArgAction::SetTrue, Parser, Subcommand, arg};
use huya_database::{
    establish_connection,
    rooms::{create, delete, read, update},
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "房间标题")]
        title: String,
        #[arg(short, long, action = SetTrue, help = "是否开播标记")]
        live: bool,
        #[arg(short, long, help = "房间封面")]
        image: String,
        #[arg(short = 's', long, help = "当前热度")]
        hot: i32,
        #[arg(short, long, help = "主播id")]
        uid: i32,
        #[arg(short, long, help = "分类id")]
        cateid: i32,
    },
    U {
        #[arg(long, short, help = "房间id")]
        id: i32,
        #[arg(short, long, help = "房间标题")]
        title: Option<String>,
        #[arg(short, long, action = SetTrue, help = "是否开播标记")]
        live: Option<bool>,
        #[arg(short = 'm', long, help = "房间封面")]
        image: Option<String>,
        #[arg(short = 's', long, help = "当前热度")]
        hot: Option<i32>,
        #[arg(short, long, help = "主播id")]
        uid: Option<i32>,
        #[arg(short, long, help = "分类id")]
        cateid: Option<i32>,
    },
    D {
        #[arg(short, long, help = "删除指定的房间")]
        id: i32,
    },
    R {
        #[arg(long, short, help = "查询的房价id， 默认列出全部房间")]
        id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut conn = establish_connection();

    let args = Args::parse();
    match args.cmd {
        Some(Command::C {
            title,
            live: is_live,
            image: img_url,
            hot,
            uid: user_id,
            cateid,
        }) => {
            create(&mut conn, title, is_live, img_url, hot, user_id, cateid)?;
        }
        Some(Command::R { id }) => {
            read(&mut conn, id)?;
        }
        Some(Command::U {
            id,
            title,
            live,
            image,
            hot,
            uid,
            cateid,
        }) => {
            update(&mut conn, id, title, live, image, hot, uid, cateid)?;
        }
        Some(Command::D { id }) => {
            delete(&mut conn, id)?;
        }
        _ => {
            println!("INVALID");
        }
    }

    Ok(())
}
