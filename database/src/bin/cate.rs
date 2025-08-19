use anyhow::anyhow;
use clap::{Parser, Subcommand, arg};
use database::categories::{create, delete, read, update};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    C {
        #[arg(short = 'c', long)]
        icon: Option<String>,
        #[arg(short, long)]
        big_icon: Option<String>,
        #[arg(short, long, required = true)]
        name: String,
        #[arg(short, long)]
        total: Option<i32>,
    },
    U {
        #[arg(short, long)]
        id: i32,
        #[arg(short = 'c', long)]
        icon: Option<String>,
        #[arg(short, long)]
        big_icon: Option<String>,
        #[arg(short, long)]
        name: Option<String>,
        #[arg(short, long)]
        total: Option<i32>,
    },
    D {
        #[arg(short, long)]
        id: i32,
    },
    R,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use database::establish_connection;
    let conn = &mut establish_connection();
    let args = Args::parse();
    match args.cmd {
        Some(Command::C {
            icon,
            name,
            total,
            big_icon,
        }) => {
            create(conn, icon, big_icon, name, total)?;
        }
        Some(Command::U {
            id,
            icon,
            big_icon,
            name,
            total,
        }) => {
            update(conn, id, icon, big_icon, name, total)?;
        }
        Some(Command::D { id }) => {
            delete(conn, id)?;
        }
        Some(Command::R) => {
            read(conn)?;
        }
        _ => return Err(anyhow!("未定义的指令")),
    }
    Ok(())
}
