//! # FintradeX Asset Rate Configuration
//!
//! This module configures the asset rate functionality for the FintradeX parachain,
//! providing exchange rate management and asset pricing mechanisms.
//!
//! ## Features
//!
//! - **Exchange Rate Management**: Dynamic asset pricing
//! - **Multi-Asset Support**: Support for various asset types
//! - **Governance Control**: Community-controlled rate updates
//! - **Trading Integration**: Seamless integration with trading operations
//!
//! ## Configuration
//!
//! - Asset rate pallet configuration
//! - Governance mechanisms for rate updates
//! - Currency integration
//! - Event handling
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{AccountId, Balances, Runtime, RuntimeEvent};
use frame_system::EnsureRoot;
impl pallet_asset_rate::Config for Runtime {
    type CreateOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRoot<AccountId>;
    type UpdateOrigin = EnsureRoot<AccountId>;
    type Currency = Balances;
    type AssetKind = u32;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_asset_rate::weights::SubstrateWeight<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}
