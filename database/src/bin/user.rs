use clap::{Parser, Subcommand, arg};
use huya_database::users::{create, delete, read, update};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "用户名")]
        name: String,
        #[arg(short = 'a', long, help = "头像的URL")]
        head: Option<String>,
    },
    R {
        #[arg(short, long, help = "用户id/全部")]
        id: Option<i32>,
    },
    U {
        #[arg(short, long, help = "用户id")]
        id: i32,
        #[arg(short, long, help = "用户名")]
        name: Option<String>,
        #[arg(short = 'a', long, help = "头像的URL")]
        head: Option<String>,
    },
    D {
        #[arg(short, long, help = "用户id")]
        id: i32,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use huya_database::establish_connection;

    let conn = &mut establish_connection();

    match Args::parse().cmd {
        Some(Command::C {
            name: input_name,
            head: input_head,
        }) => {
            create(conn, input_name, input_head)?;
        }
        Some(Command::U {
            id: input_id,
            name: input_name,
            head: input_head,
        }) => {
            update(conn, input_id, input_name, input_head)?;
        }
        Some(Command::R { id: input_id }) => {
            read(conn, input_id)?;
        }
        Some(Command::D { id: input_id }) => {
            delete(conn, input_id)?;
        }
        _ => panic!("Invalid command"),
    }
    Ok(())
}
