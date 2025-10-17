//! # FintradeX Parachain Node
//!
//! The FintradeX parachain node provides a high-performance blockchain node for the FintradeX
//! decentralized trading platform. This node supports:
//!
//! - **EVM Compatibility**: Full Ethereum Virtual Machine support
//! - **Cross-Chain Operations**: Hyperbridge-powered interoperability
//! - **High-Performance Trading**: Optimized for institutional-grade trading operations
//! - **RPC & WebSocket APIs**: Comprehensive API support for trading applications
//!
//! ## Features
//!
//! - Collator functionality for parachain consensus
//! - Ethereum-compatible RPC endpoints
//! - Cross-chain asset management
//! - Advanced trading infrastructure
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod command;
mod eth;
mod rpc;

fn main() -> sc_cli::Result<()> {
    command::run()
}
