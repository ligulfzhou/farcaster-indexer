mod rpc;
mod subcommands;

use clap::{Parser, Subcommand};
use dotenv::dotenv;
use service::{
    mutation, query,
    sea_orm::{Database, DbConn},
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
    // config:
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Backfill { max_fid: Option<i32> },
    Run,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let arg = Args::parse();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let db = Database::connect(database_url)
        .await
        .expect("database connection failed.");

    match arg.cmd {
        Commands::Backfill { max_fid } => {
            // dbg!(max_fid);
            let _ = subcommands::backfill::run(max_fid.unwrap_or(0)).await?;
        }
        Commands::Run => {
            dbg!("...run...");
            let _ = subcommands::run::run().await?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
