use clap::{Parser, Subcommand};

use crate::{
    crypto::{genkey, load_keys},
    server, DEFAULT_HOST,
};
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CLI {
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Genkey {
        #[arg(long)]
        name: String,
    },
    Vote {
        #[arg(short = 'n', long)]
        number: u32,
        #[arg(short = 'p', long)]
        pubkey: String,
        #[arg(short = 's', long)]
        host: Option<String>,
    },
    Serve {
        #[arg(long)]
        pubkey: String,
        #[arg(long)]
        host: Option<String>,
    },
}

pub fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Genkey { name }) => {
            cmd_genkey(name);
        }
        Some(Commands::Vote {
            number,
            pubkey,
            host,
        }) => {
            cmd_vote(*number, pubkey.clone(), host.clone());
        }
        Some(Commands::Serve { pubkey, host }) => {
            cmd_serve(pubkey, host.as_ref().unwrap_or(&DEFAULT_HOST.to_string()));
        }
        None => {}
    }
}

pub fn cmd_vote(number: u32, publickey: String, host: Option<String>) {
    println!(
        "Voting for {} with public key {} on host {:?}",
        number, publickey, host
    );
}

pub fn cmd_serve(pubkey: &String, host: &String) {
    println!("Serving on host {:?}", host);
    // load the key
    let (pubkey, privkey) = load_keys(pubkey.clone());
    // start the server
    server::serve(pubkey, privkey, host.to_string());
}

pub fn cmd_genkey(name: &String) {
    println!("Generating key pair for {}", name);

    // Generate a key pair in PKCS#8 DER format
    genkey(name.clone());

    // let signing_key = SigningKey::random(&mut OsRng);
    // let verifying_key = VerifyingKey::from(&signing_key);

    // // Convert the keys to byte arrays
    // let public_key_bytes = verifying_key.to_encoded_point().as_bytes();
    // let secret_key_bytes = signing_key.to_bytes().as_slice();

    // let path_pub = format!("pk_rsa_{}.pub", name);
    // let path_priv = format!("pk_rsa_{}.priv", name);
    // write(path_pub, public_key_bytes).expect("Failed to write public key to
    // file"); write(path_priv, secret_key_bytes).expect("Failed to write
    // secret key to file");
}
