use clap::{Parser, Subcommand, arg};
use database::users::{create, delete, login_by_id_or_phone, read, register, update};

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
        #[arg(short = 'p', long, help = "用户手机号")]
        phone: String,
        #[arg(short = 'd', long, help = "用户密码")]
        password: Option<String>,
    },
    R {
        #[arg(short, long, help = "用户id/全部")]
        id: Option<i32>,
    },
    E {
        #[arg(short, long, help = "用户手机号")]
        phone: String,
    },
    U {
        #[arg(short, long, help = "用户id")]
        id: i32,
        #[arg(short, long, help = "用户名")]
        name: Option<String>,
        #[arg(short = 'a', long, help = "头像的URL")]
        head: Option<String>,
        #[arg(short = 'p', long, help = "用户手机号")]
        phone: Option<String>,
        #[arg(short = 'd', long, help = "用户密码")]
        password: Option<String>,
    },
    D {
        #[arg(short, long, help = "用户id")]
        id: i32,
    },
    L {
        #[arg(short, long, help = "用户id/手机号")]
        id_or_phone: i32,
        #[arg(short, long, help = "密码")]
        pwd: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use database::establish_connection;

    let conn = &mut establish_connection();

    match Args::parse().cmd {
        Some(Command::L { id_or_phone, pwd }) => {
            login_by_id_or_phone(conn, id_or_phone, pwd)?;
        }
        Some(Command::C {
            name: input_name,
            head: input_head,
            phone: input_phone,
            password: input_password,
        }) => {
            create(conn, input_name, input_head, input_phone, input_password)?;
        }
        Some(Command::E { phone }) => {
            if let Err(er) = register(conn, phone, None) {
                println!("{:?}", er);
            }
        }
        Some(Command::U {
            id: input_id,
            name: input_name,
            head: input_head,
            phone: input_phone,
            password: input_password,
        }) => {
            update(
                conn,
                input_id,
                input_name,
                input_head,
                input_phone,
                input_password,
            )?;
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
