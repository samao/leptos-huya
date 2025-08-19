use clap::{Parser, Subcommand, arg};
use database::tags::{create, delete, read, update};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "Tag的标题")]
        title: String,
    },
    R {
        #[arg(short, long, help = "数据id")]
        id: Option<i32>,
    },
    U {
        #[arg(short, long, help = "数据id")]
        id: i32,
        #[arg(short, long, help = "Tag的标题")]
        title: String,
    },
    D {
        #[arg(short, long, help = "数据id")]
        id: i32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use database::establish_connection;
    let args = Args::parse();
    let conn = &mut establish_connection();
    match args.cmd {
        Some(Command::C { title }) => {
            create(conn, title)?;
        }
        Some(Command::U { id, title }) => {
            update(conn, id, title)?;
        }
        Some(Command::R { id }) => {
            read(conn, id)?;
        }
        Some(Command::D { id }) => {
            delete(conn, id)?;
        }
        _ => panic!("Invalid command"),
    }

    Ok(())
}
