use ::account::apis::configuration::ApiKey;
use account::{query_finance_balance, query_free_asset, query_funding_asset, query_trade_account_asset, AuthInfo};
use clap::Parser;
use log::info;
use fundings::apis::fundings_api;
use gecko::get_coins_map;
mod account;
mod model;
mod gecko;
#[derive(Parser, Debug)]
#[clap(
    name = "query",
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Query command line tool"
)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser, Debug)]
enum Commands {
    Account {
        #[clap(long, env = "APIKEY")]
        apikey: String,
        #[clap(long, env = "PASS_PHRASE")]
        passphrase: String,
        #[clap(long, env = "SECRET_KEY")]
        secretkey: String,
        #[clap(long)]
        funding: bool,
        #[clap(long)]
        trade: bool,
        #[clap(long)]
        finance: bool,
        #[clap(long,default_value = "true")]
        free: bool,
    },
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Account {
            apikey,
            passphrase,
            secretkey,
            funding,
            trade,
            finance,
            free,
        } => {
            let auth=AuthInfo::new(apikey.clone(), passphrase.clone(), secretkey.clone(), "https://aws.okx.com".to_string());

            if *funding {
                info!("Querying funding assets");
                query_funding_asset(&auth);
            }
            if *trade {
                info!("Querying trade account assets");
                query_trade_account_asset(&auth);
            }
            if *finance {
                info!("Querying finance balance");
                query_finance_balance(&auth);
            }
            if *free {
                info!("Querying free assets");
                let r=query_free_asset(&auth);
                if let Err(e)=r{
                    println!("query_free_asset error: {}", e);
                    return;
                }
            }
        }
    }
}
