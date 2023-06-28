use rsa::{RsaPrivateKey, RsaPublicKey};
use warp::Filter;

#[tokio::main]
pub async fn serve(pubkey: RsaPublicKey, privkey: RsaPrivateKey, host: String) {
    let home = warp::path::end().map(home);

    let vote = warp::path!("vote" / u32 / String).map(vote);

    let routes = vote.or(home);

    println!("Serving on host {:?}, with pubkey {:?}", host, pubkey);

    let addr = host.parse::<std::net::SocketAddr>().unwrap();
    warp::serve(routes).run(addr).await;
}

fn vote(number: u32, publickey: String) -> String {
    format!("Voting for {} with public key {}", number, publickey)
}

fn home() -> String {
    format!("Home page")
}
