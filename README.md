# WalChain

WalChain is an experimental network built by Blockchain entrepreneurs from Wallonia (Belgium) :belgium:

This repository contains the source code of the WalChain Blockchain node & runtime, based on [Substrate](https://www.substrate.dev/), an open-source framework for building tailored Blockchain solutions.

## Getting Started

Follow these steps to get started.

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the template node:

```sh
cargo run --release -- --dev --tmp
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/walchain -h
```

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/walchain --dev
```

Purge the development chain's state:

```bash
./target/release/walchain purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/walchain -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/walchain --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/walchain --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/walchain purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```

## License

Walchain is licensed under [Apache 2](LICENSE).
