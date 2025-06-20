<div align="center">

# FintradeX Parachain

> FintradeX is a decentralized financial trading platform built as a parachain on the Polkadot network.

</div>

## Table of Contents

- [Intro](#intro)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
- [Starting a Development Chain](#starting-a-development-chain)
  - [Omni Node](#omni-node-prerequisites)
  - [Zombienet setup with Omni Node](#zombienet-setup-with-omni-node)
  - [FintradeX Node](#fintradex-node)
  - [Takeaways](#takeaways)
- [Runtime development](#runtime-development)
- [Contributing](#contributing)

## Intro

- ⏫ FintradeX is a parachain designed for decentralized financial trading.

- 🔧 The FintradeX runtime is configured with custom pallets for financial trading functionality, along with standard pallets
for account management and balances.

- 💱 FintradeX enables secure, decentralized trading of digital assets with advanced financial instruments.

## Project Structure

The FintradeX parachain project consists of:

- 🧮 the [Runtime](./runtime/README.md) - the core logic of the FintradeX parachain.
- 💿 a [Node](./node/README.md) - the binary application for running FintradeX nodes.

## Getting Started

- 🦀 The project is using the Rust language.

- 👉 Check the
[Rust installation instructions](https://www.rust-lang.org/tools/install) for your system.

- 🛠️ Depending on your operating system and Rust version, there might be additional
packages required to compile this project - please take note of the Rust compiler output.

Fetch FintradeX code:

```sh
git clone https://github.com/fintradev/fintradex.git fintradex-parachain

cd fintradex-parachain
```

## Starting a Development Chain

### Omni Node Prerequisites

Omni Node can be used to run the FintradeX parachain's runtime for development purposes.

#### Install `polkadot-omni-node`

Please see the installation section at [`crates.io/omni-node`](https://crates.io/crates/polkadot-omni-node).

#### Build `fintradex-runtime`

```sh
cargo build --release
```

#### Install `staging-chain-spec-builder`

Please see the installation section at [`crates.io/staging-chain-spec-builder`](https://crates.io/crates/staging-chain-spec-builder).

#### Use `chain-spec-builder` to generate the `chain_spec.json` file

```sh
chain-spec-builder create --relay-chain "rococo-local" --para-id 1000 --runtime \
    target/release/wbuild/fintradex-runtime/fintradex_runtime.wasm named-preset development
```

**Note**: For FintradeX, the `para-id` must be set to `1000`, which is the value injected through the ParachainInfo
pallet into the `fintradex-runtime`'s storage.

#### Run Omni Node

Start Omni Node with the generated chain spec in development mode:

```bash
polkadot-omni-node --chain <path/to/chain_spec.json> --dev --dev-block-time 1000
```

For production-like testing, you'll need to setup a local relay chain network. We use zombienet for this setup.

### Zombienet setup with Omni Node

To setup a complete development environment with relay chain support:

#### Relay chain prerequisites

Download the required binaries from the official releases. Then expose them on `PATH`:

```sh
export PATH="$PATH:<path/to/binaries>"
```

#### Update `zombienet-omni-node.toml` with a valid chain spec path

```toml
# ...
[[parachains]]
id = 1000
chain_spec_path = "<TO BE UPDATED WITH A VALID PATH>"
# ...
```

#### Start the network

```sh
zombienet --provider native spawn zombienet-omni-node.toml
```

### FintradeX Node

The `node` crate provides an alternative way to run FintradeX nodes.

#### Install the `fintradex-node`

```sh
cargo install --path node
```

#### Setup and start the network

For setup, please consider the instructions for zombienet installation and relay chain prerequisites.

Start the network:

```sh
zombienet --provider native spawn zombienet.toml
```

### Takeaways

Development parachains:

- 🔗 Connect to relay chains for block finalization.
- 🧹 Do not persist the state in development mode.
- 💰 Are preconfigured with a genesis state that includes several prefunded development accounts.
- 🧑‍⚖️ Development accounts are used as validators, collators, and `sudo` accounts.

## Runtime development

For runtime development, you can use `OmniNode` with the `--dev` flag for a simplified development environment.

### Build a raw chain spec

Build the `fintradex-runtime` and generate a raw chain spec:

```sh
chain-spec-builder create --raw-storage --relay-chain "rococo-local" --para-id 1000 --runtime \
    target/release/wbuild/fintradex-runtime/fintradex_runtime.wasm named-preset development
```

### Alternatives

`OmniNode` can be used for runtime development if using the `--dev` flag, while `fintradex-node` doesn't
support it at this moment. It can still be used to test a runtime in a full setup where it is started alongside a
relay chain network.

## Contributing

- 🔄 This project has been customized for FintradeX's financial trading platform.

- ➡️ Any pull requests should be directed to this repository.

- 😇 Please refer to the FintradeX
[contribution guidelines](https://github.com/fintradev/fintradex/blob/main/CONTRIBUTING.md) and
[Code of Conduct](https://github.com/fintradev/fintradex/blob/main/CODE_OF_CONDUCT.md).
