use clap::{Parser, Subcommand, arg};
use huya_database::{
    establish_connection,
    rooms_tags::{create, delete, read},
};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "房间id")]
        room_id: i32,
        #[arg(short, long, help = "Tag id")]
        tag_id: i32,
    },
    R {
        #[arg(short, long, help = "房间id")]
        room_id: Option<i32>,
        #[arg(short, long, help = "Tag id")]
        tag_id: Option<i32>,
    },
    D {
        #[arg(short, long, help = "房间id")]
        room_id: Option<i32>,
        #[arg(short, long, help = "Tag id")]
        tag_id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let conn = &mut establish_connection();
    match args.cmd {
        Some(Command::C {
            room_id: rid,
            tag_id: tid,
        }) => {
            create(conn, rid, tid)?;
        }
        Some(Command::R {
            room_id: rid,
            tag_id: tid,
        }) => {
            read(conn, rid, tid)?;
        }
        Some(Command::D {
            room_id: rid,
            tag_id: tid,
        }) => {
            delete(conn, rid, tid)?;
        }
        _ => panic!("Invalid command"),
    }

    Ok(())
}
