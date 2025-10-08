//! # FintraDex Utility Configuration
//!
//! This module configures the utility pallet for the FintraDex parachain,
//! providing batch transaction processing and multi-signature operations.
//!
//! ## Features
//!
//! - **Batch Transactions**: Execute multiple transactions atomically
//! - **Multi-Signature**: Multi-signature account operations
//! - **Proxy Accounts**: Account delegation and management
//! - **Trading Optimization**: Efficient batch trading operations
//!
//! ## Configuration
//!
//! - Batch transaction limits and constraints
//! - Multi-signature parameters
//! - Proxy account settings
//! - Event handling and logging
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{
    Runtime, RuntimeEvent,RuntimeCall,OriginCaller
};
impl pallet_utility::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = pallet_utility::weights::SubstrateWeight<Runtime>;
}