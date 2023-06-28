#![no_main]

risc0_zkvm::guest::entry!(main);

use risc0_zkvm::guest::env;
use zktables_core::{Output, Vote, Votes};

pub fn main() {
    let votes: Votes = env::read();

    let mut avg = 0;
    for vote in &votes {
        avg += vote;
    }
    avg /= votes.len() as Vote;

    let out = Output { result: avg };
    env::commit(&out);
}
