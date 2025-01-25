use ::account::apis::configuration::ApiKey;
use account::{query_finance_balance, query_free_asset, query_funding_asset, query_trade_account_asset, AuthInfo};
use clap::Parser;
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
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Account {
            apikey,
            passphrase,
            secretkey,
        } => {
            let auth=AuthInfo::new(apikey.clone(), passphrase.clone(), secretkey.clone(), "https://aws.okx.com".to_string());

            query_funding_asset(&auth);
            query_trade_account_asset(&auth);
            query_finance_balance(&auth);

            let r=query_free_asset(&auth);
            if let Err(e)=r{
                println!("query_free_asset error: {}", e);
                return;
            }
            
        }
    }
}
