use account::apis::account_api;
use clap::Parser;



#[derive(Parser, Debug)]
#[clap(name = "query", version = "1.0", author = "Your Name <your.email@example.com>", about = "Query command line tool")]
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
#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Account { apikey, passphrase, secretkey } => {
            
            //TODO: set base
           let conf= account::apis::configuration::Configuration::new("https://aws.okx.com", apikey, secretkey, passphrase);
            let res=account_api::api_v5_account_balance_get(&conf, None).await;
            // Execute account query logic here
            println!("Account Query Result: {:?}", res);
        }
    }
}