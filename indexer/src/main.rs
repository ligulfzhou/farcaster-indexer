use clap::{Parser, Subcommand};
mod subcommands;
mod rpc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Backfill { max_fid: Option<i32> },
    Run,
}

#[tokio::main]
async fn main() {
    let arg = Args::parse();

    match arg.cmd {
        Commands::Backfill { max_fid } => {
            // dbg!(max_fid);
            subcommands::backfill::run(max_fid.unwrap_or(0)).await;
        }
        Commands::Run => {
            dbg!("...run...");
            subcommands::run::
        }
    }
}
