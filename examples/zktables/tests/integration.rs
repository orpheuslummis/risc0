use std::{
    env, fs,
    process::{Child, Command},
    thread,
};

use reqwest::Client;
use zktables::execute_and_prove;
use zktables_core::Votes;
use zktables_methods::ZKTABLES_GUEST_ID;

const N: usize = 10;

#[test]
fn ok() { // this is calling code directly
          // generate n clients
          // each vote to server (http post) -- ie sending their encrypted vote
          // server computes round -- ie decrypts all votes and computes
          // f(votes) and outputs a Vec<User,EncResult> we query (http
          // get) and ensure the resulted json is correct we do so by
          // using decryption key of each user to decrypt the result
}

// NOTE we can also test smaller things, like units test or partial integration
// tests

// -------------------

// pre-generate the random votes
// server: keygen `cargo run -- keygen server`
// server: serve `cargo run -- serve --pk name`
// for i in 0..10 clients {
// client: keygen `cargo run -- keygen client$i`
// client: vote `cargo run -- vote --pk name --host host`
// }
// we assert that the computed f(votes) is correct
#[test]
fn test_from_cli() {
    // TBD
    let feature = if cfg!(feature = "metal") {
        "metal"
    } else if cfg!(feature = "cuda") {
        "cuda"
    } else {
        "default"
    }
    .to_string();

    // Build the project -- tests run more effectively this way
    let out = Command::new("cargo")
        .args(["build", "--out-dir", "build", "-Z", "unstable-options"])
        .output()
        .unwrap();
    check_output_success(out);
    const PROGRAM_PATH: &str = "./build/zktables";

    // Generate a random hash as server key name
    let server_key_name = format!("{:x}", rand::random::<u32>());

    // Create a temporary directory
    let mut dir = env::temp_dir();
    dir.push("keys/");
    println!("Temp dir: {:?}", dir);
    fs::create_dir_all(dir.clone()).expect("Failed to create temp directory");
    let dir = dir.to_str().unwrap().to_string();

    let out = Command::new(PROGRAM_PATH)
        .args([
            "keygen",
            "--name",
            &server_key_name,
            "--keydir",
            dir.clone().as_str(),
        ])
        .output()
        .unwrap();

    println!(
        "server keygen stdout: {}",
        String::from_utf8_lossy(&out.stdout)
    );
    println!(
        "server keygen stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    check_output_success(out);

    // run zktables keyget --name server_key_name --keydir dir
    // and capture stdout
    let out = Command::new(PROGRAM_PATH)
        .args([
            "keyget",
            "--name",
            &server_key_name,
            "--keydir",
            dir.clone().as_str(),
        ])
        .output()
        .unwrap();
    let server_pubkey = String::from_utf8_lossy(&out.stdout);
    println!("server pubkey: {:?}", server_pubkey);

    let mut server_handle = run_server(
        PROGRAM_PATH.to_string(),
        feature,
        server_key_name.clone(),
        dir.clone(),
    );

    let mut clients = Vec::new();
    for i in 0..N {
        // Generate a random hash as key name
        let key_name = format!("{:x}", rand::random::<u32>());
        let client = TestClient {
            key_name: key_name.clone(),
        };
        clients.push(client.clone());

        let out = Command::new(PROGRAM_PATH)
            .args([
                "keygen",
                "--name",
                &client.key_name,
                "--keydir",
                dir.clone().as_str(),
            ])
            .output()
            .unwrap();
        check_output_success(out);

        let out = Command::new(PROGRAM_PATH)
            .args([
                "vote",
                "--vote",
                i.to_string().as_str(),
                "--key",
                &client.key_name.as_str(),
                "--server.pubkey",
                server_key_name.as_str(),
                "--keydir",
                dir.clone().as_str(),
            ])
            .output()
            .unwrap();
        check_output_success(out);
    }

    // sleep 1 seconds
    thread::sleep(std::time::Duration::from_secs(1));
    println!("Server output: {:?}", server_handle);

    server_handle.kill().expect("failed to kill server");
}

fn run_server(program_path: String, feature: String, pubkey: String, keydir: String) -> Child {
    let server = thread::spawn(move || {
        Command::new(program_path)
            .args([
                "serve",
                "--pubkey",
                pubkey.as_str(),
                "--keydir",
                keydir.as_str(),
            ])
            .spawn()
            .expect("server failed to start")
    });
    server.join().expect("server thread panicked")
}

fn check_output_success(output: std::process::Output) {
    if !output.status.success() {
        println!("Command executed with errors:");
        println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        panic!("Command did not execute successfully");
    }
}

#[derive(Clone)]
struct TestClient {
    pub key_name: String,
}
