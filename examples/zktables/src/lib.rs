use risc0_zkvm::{
    serde::{from_slice, to_vec},
    Executor, ExecutorEnv,
};
use zktables_core::{Output, Votes};
use zktables_methods::{ZKTABLES_GUEST_ELF, ZKTABLES_GUEST_ID};

pub mod cli;
pub mod crypto;
pub mod db;
pub mod server;

// Default configuration
pub const DEFAULT_SEED: u64 = 0;
pub const DEFAULT_NBITS: usize = 2048;
pub const DEFAULT_HOST: &str = "127.0.0.1:3030";
pub const DEFAULT_PATH: &str = "./keys/";

pub fn execute_and_prove(votes: Votes) -> (Box<dyn risc0_zkvm::receipt::SessionReceipt>, Output) {
    let env = ExecutorEnv::builder()
        .add_input(&to_vec(&votes).unwrap())
        .build()
        .unwrap();

    let mut exec = Executor::from_elf(env, ZKTABLES_GUEST_ELF).unwrap();

    let session = exec.run().unwrap();

    let receipt = session.prove().unwrap();

    receipt.verify(ZKTABLES_GUEST_ID.into()).unwrap();

    let output = from_slice::<Output, _>(&receipt.get_journal()).unwrap();

    (receipt, output)
}
