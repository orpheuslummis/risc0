use reqwest::blocking::Client;
use rsa::pkcs1::DecodeRsaPublicKey;
use zktables_core::{crypto, Vote};

use crate::DEFAULT_HOST;

// TODO
pub fn vote(vote: Vote, key: String, server_pubkey: String, host: Option<String>, keydir: String) {
    let host = host.unwrap_or(DEFAULT_HOST.to_string());
    println!(
        "Voting for {:?} using key {:?} for host {:?} with public key {:?}",
        vote, key, host, server_pubkey
    );
    let (pubkey, privkey) = crypto::load_keys(key.clone(), keydir);

    println!("server pubkey: {:?}", server_pubkey);

    let server_pubkey_object = rsa::RsaPublicKey::from_pkcs1_pem(&server_pubkey).unwrap();

    let encrypted = crypto::encrypt(&vote, server_pubkey_object);

    let client = Client::new();
    let res = client
        .post(&format!("http://{}/vote", host))
        .body(encrypted)
        .send()
        .unwrap();

    if res.status().is_success() {
        println!("Vote successful");
    } else {
        println!("Vote failed");
    }
}
