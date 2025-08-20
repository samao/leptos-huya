use clap::{Parser, Subcommand, arg};
use database::tokens::{clear_user, create, delete, get_user, read};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short, long, help = "用户id")]
        id: i32,
        #[arg(short, long, help = "登录token")]
        token: String,
    },
    R {
        #[arg(short, long, help = "用户id")]
        id: Option<i32>,
        #[arg(short, long, help = "登录token")]
        token: Option<String>,
    },
    D {
        #[arg(short, long, help = "登录token")]
        token: Option<String>,

        #[arg(short, long, help = "登录token 的uid")]
        id: Option<i32>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use database::establish_connection;
    let args = Args::parse();
    let conn = &mut establish_connection();
    match args.cmd {
        Some(Command::C { id, token }) => {
            create(conn, id, token)?;
        }
        Some(Command::R { id, token }) => {
            if id.is_some() {
                read(conn, id)?;
            } else if let Some(token) = token {
                get_user(conn, token)?;
            }
        }
        Some(Command::D { token, id }) => {
            if let Some(token) = token {
                delete(conn, token)?;
            }

            if let Some(id) = id {
                clear_user(conn, id)?;
            }
        }
        _ => panic!("Invalid command"),
    }

    Ok(())
}
