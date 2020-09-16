mod cli;
mod db;
mod models;
mod services;
mod util;

use cli::*;
use services::run;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opts: Opts = get_opts();

    match opts.subcmd {
        SubCommand::Run(_) => {
            run().await.unwrap_or_else(|err| eprintln!("{}", err));
        }
        SubCommand::Db(d) => match d.subcmd {
            DbSubCommand::Create(_) => {
                db::create()
                    .await
                    .unwrap_or_else(|err| eprintln!("{}", err));
            }
            DbSubCommand::Init(_) => {
                db::init().await.unwrap_or_else(|err| eprintln!("{}", err));
            }
            DbSubCommand::Drop(_) => {
                db::drop().await.unwrap_or_else(|err| eprintln!("{}", err));
            }
            DbSubCommand::Mock(_) => {
                db::create_mock()
                    .await
                    .unwrap_or_else(|err| eprintln!("{}", err));
            }
        },
    }

    Ok(())
}
