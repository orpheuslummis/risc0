use std::process::Command;

use zktables::execute_and_prove;
use zktables_core::Votes;
use zktables_methods::ZKTABLES_GUEST_ID;

#[test]
fn ok() {
    let votes: Votes = vec![1, 2, 3];
    let (receipt, output) = execute_and_prove(votes);
    let v = receipt.verify(ZKTABLES_GUEST_ID.into());
    assert!(v.is_ok());
    assert_eq!(output.result, 2);
}

#[test]
fn test_fromc_cli() {
    // let feature = if cfg!(feature = "metal") {
    //     "metal"
    // } else if cfg!(feature = "cuda") {
    //     "cuda"
    // } else {
    //     "default"
    // };

    // pre-generate the random votes
    // server: genkey `cargo run -- genkey server`
    // server: serve `cargo run -- serve --pk name`
    // for i in 0..10 clients {
    // client: genkey `cargo run -- genkey client$i`
    // client: vote `cargo run -- vote --pk name --host host`
    // }
    // we assert that the computed f(votes) is correct
}
