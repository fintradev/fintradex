<div align="center">

# FintradeX Parachain

> **The Future of Decentralized Financial Trading on Polkadot**

[![FintradeX](https://img.shields.io/badge/FintradeX-Financial%20Trading-blue?style=for-the-badge&logo=polkadot)](https://fintradex.io/)
[![Polkadot SDK](https://img.shields.io/badge/Polkadot%20SDK-Stable%202503-green?style=for-the-badge)](https://github.com/paritytech/polkadot-sdk)
[![Rust](https://img.shields.io/badge/Rust-1.75+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)

**A high-performance, cross-chain decentralized trading platform built on Polkadot**

</div>

## 🚀 Vision

FintradeX is revolutionizing decentralized finance by creating the most advanced trading infrastructure on Polkadot. We're building a platform that combines the speed and efficiency of traditional financial markets with the transparency and accessibility of blockchain technology.

### 🎯 Mission
- **Democratize Trading**: Make professional-grade trading tools accessible to everyone
- **Cross-Chain Liquidity**: Unify liquidity across multiple blockchain networks
- **Institutional-Grade Infrastructure**: Provide enterprise-level trading capabilities
- **Community-Driven Governance**: Empower users to shape the future of DeFi trading

## 🌟 Key Features

### 🏦 Advanced Trading Infrastructure
- **Multi-Asset Trading**: Trade any asset across multiple blockchains
- **High-Frequency Trading**: Sub-second order execution with minimal latency
- **Advanced Order Types**: Limit, market, stop-loss, and conditional orders
- **Real-Time Market Data**: Live price feeds and market analytics
- **Liquidity Pools**: Automated market making with deep liquidity

### 🔗 Cross-Chain Capabilities
- **Unified Trading Experience**: Trade assets from any connected blockchain
- **Seamless Asset Transfers**: Instant cross-chain asset movement
- **Interoperable Liquidity**: Share liquidity across the entire Polkadot ecosystem
- **Multi-Chain Order Books**: Unified order books across multiple networks

### 🛡️ Security & Compliance
- **Institutional-Grade Security**: Enterprise-level security protocols
- **Regulatory Compliance**: Built-in compliance features for institutional adoption
- **Audit-Ready Infrastructure**: Transparent and auditable trading operations
- **Risk Management**: Advanced risk controls and position monitoring

### 🏛️ Governance & Economics
- **DAO Governance**: Community-driven protocol decisions
- **Fee Distribution**: Fair and transparent fee sharing mechanisms
- **Staking Rewards**: Earn rewards by participating in network security
- **Treasury Management**: Community-controlled development funding

## 📊 Trading Features

### Spot Trading
- **Instant Settlement**: T+0 settlement for all trades
- **Deep Liquidity**: Access to liquidity across multiple chains
- **Advanced Charts**: Professional-grade trading charts and indicators
- **Portfolio Management**: Comprehensive portfolio tracking and analytics

### Derivatives Trading
- **Futures & Options**: Advanced derivative instruments
- **Leverage Trading**: Flexible margin trading with risk controls
- **Synthetic Assets**: Trade any asset as synthetic derivatives
- **Risk Hedging**: Advanced hedging strategies and tools

### DeFi Integration
- **Yield Farming**: Earn rewards through liquidity provision
- **Staking Protocols**: Participate in various staking opportunities
- **Lending & Borrowing**: Access to decentralized lending markets
- **Insurance Products**: Protect against smart contract risks

## 🏗️ Architecture

The FintradeX parachain consists of:

- 🧮 **[Runtime](./runtime/README.md)** - The core trading logic and state management
- 💿 **[Node](./node/README.md)** - High-performance blockchain node for trading operations
- 🔗 **Cross-Chain Bridge** - Seamless asset transfer between blockchains
- 📊 **Market Data Engine** - Real-time price feeds and market analytics
- 🛡️ **Security Layer** - Advanced security and risk management systems

## 🚀 Getting Started

### Prerequisites
- 🦀 **Rust**: 1.86 or higher
- 📦 **Cargo**: Latest version
- 🔧 **System Dependencies**: 
  - Ubuntu/Debian: `build-essential`, `cmake`, `pkg-config`, `libssl-dev`
  - macOS: Xcode Command Line Tools
  - Windows: Visual Studio Build Tools

#### Rust Setup
Run the following commands to set up the correct Rust version:

```bash
rustup default 1.86
rustup target add wasm32-unknown-unknown --toolchain 1.86-aarch64-apple-darwin
rustup component add rust-src --toolchain 1.86-aarch64-apple-darwin
```

#### Required Tools

**Chain Spec Builder** - A Polkadot SDK utility for generating chain specifications. Refer to the [Generate Chain Specs documentation](https://docs.substrate.io/build/chain-spec/) for detailed usage.

Install it by executing the following command:

```bash
cargo install --locked staging-chain-spec-builder@10.0.0
```

This installs the `chain-spec-builder` binary.

**Polkadot Omni Node** - A white-labeled binary, released as a part of Polkadot SDK that can act as the collator of a parachain in production, with all the related auxiliary functionalities that a normal collator node has: RPC server, archiving state, etc. Moreover, it can also run the wasm blob of the parachain locally for testing and development.

To install it, run the following command:

```bash
cargo install --locked polkadot-omni-node@0.5.0
```

This installs the `polkadot-omni-node` binary.

### Technical Setup Guide

For advanced users and developers, follow these detailed steps to set up the parachain:

#### 1. Clone the Repository
```bash
git clone https://github.com/fintradev/fintradex.git
cd fintradex
```

#### 2. Compile the Runtime
```bash
cargo build --release --locked
```

#### 3. Generate Chain Specification
Create a development network chain specification file:
```bash
chain-spec-builder create -t development \
  --relay-chain paseo \
  --para-id 1000 \
  --runtime ./target/release/wbuild/fintradex-runtime/fintradex_runtime.compact.compressed.wasm \
  named-preset development
```

#### 4. Start the Omni Node
Start the node in development mode (without a relay chain config), producing and finalizing blocks:
```bash
polkadot-omni-node --chain ./chain_spec.json --dev
```

**Note**: This setup runs the parachain in standalone development mode for testing and development purposes.

### Development Environment

```bash
# Install development dependencies
cargo install --path node

# Run tests
cargo test

# Start with zombienet (recommended for development)
zombienet --provider native spawn zombienet.toml
```