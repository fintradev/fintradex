<div align="center">

# FintradeX Parachain


> This is the FintradeX parachain implementation based on Polkadot SDK.
>
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

- ⏫ FintradeX is a [parachain](https://wiki.polkadot.network/docs/learn-parachains) designed for decentralized financial trading.

- ☁️ It is based on the
[Cumulus](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/polkadot_sdk/cumulus/index.html) framework.

- 🔧 Its runtime is configured with custom pallets for financial trading functionality, along with standard pallets
such as a [Balances pallet](https://paritytech.github.io/polkadot-sdk/master/pallet_balances/index.html).

- 👉 Learn more about parachains [here](https://wiki.polkadot.network/docs/learn-parachains)

## Project Structure

The FintradeX parachain project consists of:

- 🧮 the [Runtime](./runtime/README.md) - the core logic of the FintradeX parachain.
- 💿 a [Node](./node/README.md) - the binary application, not part of the project default-members list and not compiled unless
building the project with `--workspace` flag, which builds all workspace members, and is an alternative to
[Omni Node](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html).

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

[Omni Node](https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/reference_docs/omni_node/index.html) can
be used to run the FintradeX parachain's runtime. `polkadot-omni-node` binary crate usage is described at a high-level
[on crates.io](https://crates.io/crates/polkadot-omni-node).

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

**Note**: the `relay-chain` and `para-id` flags are mandatory information required by
Omni Node, and for FintradeX case the value for `para-id` must be set to `1000`, since this
is also the value injected through [ParachainInfo](https://docs.rs/staging-parachain-info/0.17.0/staging_parachain_info/)
pallet into the `fintradex-runtime`'s storage. The `relay-chain` value is set in accordance
with the relay chain ID where this instantiation of FintradeX will connect to.

#### Run Omni Node

Start Omni Node with the generated chain spec. We'll start it in development mode (without a relay chain config), producing
and finalizing blocks based on manual seal, configured below to seal a block with each second.

```bash
polkadot-omni-node --chain <path/to/chain_spec.json> --dev --dev-block-time 1000
```

However, such a setup is not close to what would run in production, and for that we need to setup a local
relay chain network that will help with the block finalization. In this guide we'll setup a local relay chain
as well. We'll not do it manually, by starting one node at a time, but we'll use [zombienet](https://paritytech.github.io/zombienet/intro.html).

Follow through the next section for more details on how to do it.

### Zombienet setup with Omni Node

Assuming we continue from the last step of the previous section, we have a chain spec and we need to setup a relay chain.
We can install `zombienet` as described [here](https://paritytech.github.io/zombienet/install.html#installation), and
`zombienet-omni-node.toml` contains the network specification we want to start.

#### Relay chain prerequisites

Download the `polkadot` (and the accompanying `polkadot-prepare-worker` and `polkadot-execute-worker`) binaries from
[Polkadot SDK releases](https://github.com/paritytech/polkadot-sdk/releases). Then expose them on `PATH` like so:

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

As mentioned in the `Project Structure` section, the `node` crate is optionally compiled and it is an alternative
to `Omni Node`. Similarly, it requires setting up a relay chain, and we'll use `zombienet` once more.

#### Install the `fintradex-node`

```sh
cargo install --path node
```

#### Setup and start the network

For setup, please consider the instructions for `zombienet` installation [here](https://paritytech.github.io/zombienet/install.html#installation)
and [relay chain prerequisites](#relay-chain-prerequisites).

We're left just with starting the network:

```sh
zombienet --provider native spawn zombienet.toml
```

### Takeaways

Development parachains:

- 🔗 Connect to relay chains, and we showcased how to connect to a local one.
- 🧹 Do not persist the state.
- 💰 Are preconfigured with a genesis state that includes several prefunded development accounts.
- 🧑‍⚖️ Development accounts are used as validators, collators, and `sudo` accounts.

## Runtime development

For runtime development, you can use `OmniNode` with the `--dev` flag for a simplified development environment.

### Build a raw chain spec

Build the `fintradex-runtime` as mentioned before in this guide and use `chain-spec-builder`
again but this time by passing `--raw-storage` flag:

```sh
chain-spec-builder create --raw-storage --relay-chain "rococo-local" --para-id 1000 --runtime \
    target/release/wbuild/fintradex-runtime/fintradex_runtime.wasm named-preset development
```

### Alternatives

`OmniNode` can be used for runtime development if using the `--dev` flag, while `fintradex-node` doesn't
support it at this moment. It can still be used to test a runtime in a full setup where it is started alongside a
relay chain network (see [FintradeX node](#fintradex-node) setup).

## Contributing

- 🔄 This project is based on the Polkadot SDK parachain template and has been customized for FintradeX.

- ➡️ Any pull requests should be directed to this repository.

- 😇 Please refer to the FintradeX
[contribution guidelines](https://github.com/fintradev/fintradex/blob/main/CONTRIBUTING.md) and
[Code of Conduct](https://github.com/fintradev/fintradex/blob/main/CODE_OF_CONDUCT.md).
