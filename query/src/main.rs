use ::account::apis::configuration::ApiKey;
use account::{query_account_asset, query_finance_balance, query_trade_account_asset};
use clap::Parser;
use fundings::apis::fundings_api;
mod account;
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
            query_account_asset(apikey, passphrase, secretkey);
            query_trade_account_asset(apikey, passphrase, secretkey);
            query_finance_balance(apikey, passphrase, secretkey);
        }
    }
}
