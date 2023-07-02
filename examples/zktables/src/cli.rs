use clap::{Parser, Subcommand};
use rsa::pkcs1::EncodeRsaPublicKey;
use zktables_core::{
    crypto::{self, generate_keypair, write_keys_to_dir},
    Vote,
};

use crate::{client, server, DEFAULT_HOST, DEFAULT_KEYPATH};

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
    Keygen {
        #[arg(long)]
        name: String,
        #[arg(long)]
        keydir: Option<String>,
    },
    Keyget {
        #[arg(long)]
        name: String,
        #[arg(long)]
        keydir: Option<String>,
    },
    Vote {
        #[arg(long)]
        vote: Vote,
        #[arg(long)]
        key: String,
        #[arg(long = "server.pubkey")]
        server_pubkey: String,
        #[arg(long)]
        keydir: Option<String>,
        #[arg(long)]
        host: Option<String>,
    },
    Server {
        #[arg(long)]
        host: Option<String>,
    },
}

pub fn main() {
    let cli = CLI::parse();

    match &cli.command {
        Some(Commands::Keygen { name, keydir }) => {
            cmd_keygen(
                name,
                keydir.as_ref().unwrap_or(&DEFAULT_KEYPATH.to_string()),
            );
        }
        Some(Commands::Keyget { name, keydir }) => {
            cmd_keyget(
                name,
                keydir.as_ref().unwrap_or(&DEFAULT_KEYPATH.to_string()),
            );
        }
        Some(Commands::Vote {
            vote,
            key,
            server_pubkey,
            host,
            keydir,
        }) => {
            cmd_vote(
                *vote,
                key.clone(),
                server_pubkey.clone(),
                host.clone(),
                keydir.clone().unwrap_or(DEFAULT_KEYPATH.to_string()),
            );
        }
        Some(Commands::Server { host }) => {
            cmd_server(host.as_ref().unwrap_or(&DEFAULT_HOST.to_string()));
        }
        None => {
            println!("No command specified");
        }
    }
}

pub fn cmd_server(host: &String) {
    server::server(host.to_string());
}

pub fn cmd_keygen(name: &String, keydir: &String) {
    println!("Generating key pair {} in {}", name, keydir);
    let (pubkey, privkey) = generate_keypair();
    write_keys_to_dir(name.clone(), keydir.clone(), pubkey, privkey);
}

pub fn cmd_keyget(name: &String, keydir: &String) {
    let (pubkey, _) = crypto::load_keys(name.clone(), keydir.clone());
    let pubkey_serialized = pubkey
        .0
        .to_pkcs1_pem(rsa::pkcs1::LineEnding::default())
        .unwrap();
}

pub fn cmd_vote(
    vote: Vote,
    key: String,
    server_pubkey: String,
    host: Option<String>,
    keydir: String,
) {
    client::vote(vote, key, server_pubkey, host, keydir);
}

//
