//! # FintradeX Ethereum Configuration
//!
//! This module configures the Ethereum integration for the FintradeX parachain,
//! providing full compatibility with Ethereum smart contracts and DeFi protocols.
//!
//! ## Features
//!
//! - **Ethereum Compatibility**: Full EVM support for smart contracts
//! - **DeFi Integration**: Seamless integration with Ethereum DeFi protocols
//! - **Precompiles**: Optimized cryptographic operations
//! - **Gas System**: Ethereum-compatible gas pricing and limits
//!
//! ## Configuration
//!
//! - Ethereum pallet configuration
//! - Precompile integration
//! - Gas system parameters
//! - Smart contract execution settings
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::Runtime;
pub use frame_support::{
    parameter_types,
    traits::{ConstU32, Get},
    StorageValue,
};
use pallet_ethereum::PostLogContent;
parameter_types! {
    pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
    //type RuntimeEvent = RuntimeEvent;
    type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
    type PostLogContent = PostBlockAndTxnHashes;
    type ExtraDataLength = ConstU32<30>;
}
