//

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use warp::Filter;

const DEFAULT_HOST: &str = "127.0.0.1:3030";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional name to operate on
    name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
    Vote {
        #[arg(short = 'n', long)]
        number: u32,
        #[arg(short = 'p', long)]
        publickey: String,
        #[arg(short = 's', long)]
        host: Option<String>,
    },
    Serve {},
}

fn main() {
    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.name.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.config.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => println!("Debug mode is off"),
        1 => println!("Debug mode is kind of on"),
        2 => println!("Debug mode is on"),
        _ => println!("Don't be crazy"),
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        Some(Commands::Vote {
            number,
            publickey,
            host,
        }) => {
            println!(
                "Voting for {} with public key {} on host {:?}",
                number, publickey, host
            );
        }
        Some(Commands::Serve {}) => {
            println!("Serving...");
            serve(DEFAULT_HOST);
        }
        None => {}
    }

    // Continued program logic goes here...
}

#[tokio::main]
async fn serve(host: &str) {
    let vote = warp::path!("vote" / u32 / String).map(vote);

    let home = warp::path::end().map(home);

    let routes = vote.or(home);

    let addr = host.parse::<std::net::SocketAddr>().unwrap();
    warp::serve(routes).run(addr).await;
}

fn vote(number: u32, publickey: String) -> String {
    format!("Voting for {} with public key {}", number, publickey)
}

fn home() -> String {
    format!("Home page")
}
