NOTES
-----

integration test

client(vote, key_name, server_public_key, server_location)
- encrypts data with rsa
- 

server(key_name, location)
- results stored in sqlite
- resulting given as json on /result














how to use multiple binaries with cargo run --bin program_name










warp filters like in todos

sqlite

data management
client: files
server: db and files


---

Ephemeral Diffie-Hellman exchanges
https://docs.rs/elliptic-curve/0.13.4/elliptic_curve/ecdh/struct.EphemeralSecret.html


if let Some(config_path) = cli.config.as_deref() {
    println!("Value for config: {}", config_path.display());
}

match cli.debug {
    1 => println!("Debug mode"),
    _ => (),
}


---


if it would be on defradb