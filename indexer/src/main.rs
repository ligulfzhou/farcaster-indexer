mod rabbitmq;
mod subcommands;

use clap::{Parser, Subcommand};
use lapin::{
    message::DeliveryResult,
    options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties, Queue,
};
use service::sea_orm::Database;

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
    Index,
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
            subcommands::backfill::run(&db, max_fid.unwrap_or(0))
                .await
                .expect("run backfill");
        }
        Commands::Index => {
            subcommands::index::run(&db).await.expect("run indexer");
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {}
}
