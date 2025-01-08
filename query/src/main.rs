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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Account { apikey, passphrase, secretkey } => {
            
           let conf= account::apis::configuration::Configuration::new("".to_string(), apikey, secretkey, passphrase);
            // Execute account query logic here
        }
    }
}