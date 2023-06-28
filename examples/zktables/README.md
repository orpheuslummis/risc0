# zktables

`zktables` is a zero-knowledge proof based voting system that allows participants to send their encrypted input to a centralized aggregation node. The system guarantees integrity of computation and ensures that the inputs and results are private (readable by the aggregation node only), and results only available to participants.

Participants encrypt their input with the public key of the aggregation node and send it. Optionally, they can run a verified/zkvm program to ensure integrity of the computation while obtaining input. Once a threshold or all inputs of participants have been received, the aggregation node runs a verified/zkvm program to perform function f(input). The result is encrypted with each participant's public key, ensuring that the result is only available to the group of participants.

The aggregation node is designed to run in a secure enclave such that the host can't observe the decrypted data. The voting process can proceed in rounds.

In the proof of concept:

The primary functionalities are offered via two command-line commands:

- `zktables serve`: This command is used by the aggregation node to receive votes and produce proof of voting round. Additionally, it provides information on the home page. <http://localhost:3030/> by default.

- `zktables vote`: This command is used by participants to send their vote. The command takes in three parameters: a number N which represents the vote, a publickey PK of the aggregation node and a host HOST address which is where the aggregation node is located.

- The public key scheme used is RSA (Rivest–Shamir–Adleman). It could perhaps be switched for a scheme that is quantum-ready and blockchain-compatible in the future.

- The aggregation `f(votes)` is just averaging. The `f` and `votes` can be changed for more complex input data and integration function.



## Run it locally

Ensure Rust is intalled (<https://www.rust-lang.org/tools/install>).

Clone the repository: `git clone https://github.com/username/zktables.git`.

In Terminal 1, start the service with `cargo run -- serve`. This will display a public key in the logs.

In Terminal 2, cast a vote using` cargo run -- vote --input N --publickey PK --host http://localhost:3030`, where `N` is your vote, `PK` is the public key from Terminal 1, and the host address is the location of the aggregation node.

