# SP1 Storage Proof

Quite literally the simplest and most straightforward implementation of an EVM storage proof for SP1.

## Requirements

- [Rust](https://rustup.rs/)
- [SP1](https://docs.succinct.xyz/getting-started/install.html)

## Setup

Copy the example environment file:

```sh
cp .env.example .env
```

And set the values:

- `ETH_RPC_URL`: The RPC URL of an _archive_ Ethereum node. You can use a public provider like [Infura](https://infura.io/) or run your own for to flex on the rest of us.
- [Optional] `SP1_PRIVATE_KEY`: Your private key for [Succinct's prover network](https://docs.succinct.xyz/generating-proofs/prover-network.html). You may also choose to run the program in execution mode

## Running the Project

### Build the Program

To build the program, run the following command:

```sh
cd program
cargo prove build
```

### Execute the Program

To run the program without generating a proof:

```sh
cd script
cargo run --release -- --execute
```

This will execute the program and display the cycle count, which hovers around ~130k cycles.

### Generate a Core Proof

To generate a core proof for your program:

```sh
cd script
cargo run --release -- --prove
```

### Retrieve the Verification Key

To retrieve your `programVKey` for your on-chain contract, run the following command:

```sh
cargo prove vkey --program fibonacci-program
```
