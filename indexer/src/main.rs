mod rabbitmq;
mod redis;
mod subcommands;

use clap::{Parser, Subcommand};
use farcaster_client::client::Client;
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
    ClearMQ,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let arg = Args::parse();

    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL not found.");
    let db = Database::connect(database_url)
        .await
        .expect("database connection failed.");

    let hub_url = dotenv::var("HUB_GRPC").expect("HUB_GRPC not found.");
    let hub_client = Client::new(hub_url).await.expect("HUB_GRPC not valid");

    match arg.cmd {
        Commands::Backfill { max_fid } => {
            subcommands::backfill::run(&db, hub_client, max_fid.unwrap_or(0))
                .await
                .expect("run backfill");
        }
        Commands::Index => {
            subcommands::index::run(&db, hub_client)
                .await
                .expect("run indexer");
        }
        Commands::ClearMQ => subcommands::clear_mq::run().await,
    };
}
