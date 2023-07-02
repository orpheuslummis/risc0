# zktables

`zktables` is a zero-knowledge proof based voting system that allows participants to send their encrypted input to a centralized aggregation node. The system guarantees integrity of computation and ensures that the inputs and results are private (readable by the aggregation node only), and results only available to participants.

Participants encrypt their input with the public key of the aggregation node and send it. Optionally, they can run a verified/zkvm program to ensure integrity of the computation while obtaining input. Once a threshold or all inputs of participants have been received, the aggregation node runs a verified/zkvm program to perform function f(input). The result is encrypted with each participant's public key, ensuring that the result is only available to the group of participants.

The aggregation node is designed to run in a secure enclave such that the host can't observe the decrypted data. The voting process can proceed in rounds.

The primary functionalities:

- `zktables keygen --name <name>`: This command is to generate public and private keys, to be stored under its name in `./key/`. The keys are used by both client and server.

- `zktables keyget --name <name>`: This command is to get the public key of a previously generated keypair, to be used by the client to encrypt their vote.

- `zktables serve`: This command is used by the aggregation node to receive votes and produce proof of voting round. Additionally, it provides information on the home page. <http://localhost:3030/> by default.

- `zktables vote`: This command is used by participants to send their vote. The command takes in three parameters: a number N which represents the vote, a publickey PK of the aggregation node and a host HOST address which is where the aggregation node is located.

- The aggregation `f(votes)` is just averaging. The `f` and `votes` can be changed for more complex input data and integration function.

It is implemented in many suboptimal ways, for the sake of simplicity. It is a proof of concept.


## Run it locally

Ensure Rust is intalled (<https://www.rust-lang.org/tools/install>).

```shell
# clone the repo
git clone https://github.com/orpheuslummis/risc0
cd risc0/examples/zktables
```

In terminal 1:
```shell
cargo run -- keygen --name myexperiment
cargo run -- serve --key myexperiment
```

In terminal 2:
```shell
# where `N` is your vote, `PK` is the public key from Terminal 1
cargo run -- vote --input N --pubkey PK
```


## Notes

On tools used:

- the ZK system used is `risc0-zkvm`
- public key cryptography using `RSA`, specifically to encrypt inputs to the node, and for the node to encrypt the results to the participants
- HTTP server with `warp`
- HTTP request with `reqwest`
- CLI with `clap`
- sqlite database to store the (encrypted) results, with `rusqlite`

By default, the keys are stored in `./keys` and are referred to by name, i.e. `pk_rsa_name.pub` and `pk_rsa_name.priv`

The public key scheme used is RSA (Rivest–Shamir–Adleman). It could perhaps be switched for a scheme that is quantum-ready and blockchain-compatible in the future.

In this version we have the results the same for all participants, but in the future we could have different results for different participants.



## One run through of the protocol

Participants send their encrypted votes to `/vote`.
A round completes when N votes - which is a param defined by the server.
The round data is sent to the zkVM, such as to decrypt, compute f(votes), and encrypt results for each participant.
The server exposes global state, including results on `/.


## Questions

Server state

Guest state

A better way to do rounds? Time-bound, or whitelist of participants.