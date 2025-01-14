use account::apis::account_api;
use clap::Parser;

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
            let conf = account::apis::configuration::Configuration::new(
                "https://aws.okx.com",
                apikey,
                secretkey,
                passphrase,
            );
            let mut rt = tokio::runtime::Runtime::new().unwrap();
            let res =
                rt.block_on(async { account_api::api_v5_account_balance_get(&conf, None).await });
            // Execute account query logic here
            match res {
                Ok(response) => {
                    println!("API call successful!");

                    let res = serde_json::from_value::<account::models::BalanceResponse>(response)
                        .unwrap();
                    let mut count=0.0;
                    for v in res.data.iter() {
                        for vv in &v.details{
                            println!("{:?}",vv);
                            let num:f64=vv.eq_usd.parse().unwrap_or(0.0);
                            count+=num;

                        }
                    }
                    println!("total {}",count);
                }
                Err(e) => {
                    println!("API call failed!");
                    println!("Error fetching account balance: {:?}", e);
                }
            }
        }
    }
}
