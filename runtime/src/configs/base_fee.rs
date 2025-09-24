//! # FintradeX Base Fee Configuration
//!
//! This module configures the base fee mechanism for the FintradeX parachain,
//! providing dynamic fee adjustment based on network congestion and demand.
//!
//! ## Features
//!
//! - **Dynamic Fee Adjustment**: Automatic fee scaling based on network usage
//! - **EVM Compatibility**: Ethereum-compatible base fee mechanism
//! - **Trading Optimization**: Optimized for high-frequency trading operations
//! - **Gas Price Management**: Competitive gas pricing for DeFi protocols
//!
//! ## Configuration
//!
//! - Base fee parameters and limits
//! - Fee adjustment algorithms
//! - Gas price calculations
//! - Network congestion handling
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use sp_core::U256;
use crate::{Runtime, RuntimeEvent};
use sp_runtime::Permill;


pub use frame_support::{
	parameter_types,
	StorageValue,
};
parameter_types! {
	//pub DefaultBaseFeePerGas: U256 = U256::from(1_000_000_000);
	//pub DefaultElasticity: Permill = Permill::from_parts(125_000);
	//pub DefaultBaseFeePerGas: U256 = U256::from(2_000_000_000);
	pub DefaultBaseFeePerGas: U256 = U256::from(0);
	pub DefaultElasticity: Permill = Permill::zero();
}
pub struct BaseFeeThreshold;
impl pallet_base_fee::BaseFeeThreshold for BaseFeeThreshold {
	fn lower() -> Permill {
		Permill::zero()
	}
	fn ideal() -> Permill {
		//Permill::from_parts(500_000)
		//Permill::from_parts(1_000_000)
		Permill::zero()
	}
	fn upper() -> Permill {
		//Permill::from_parts(2_000_000)
		Permill::zero()
	}
}
impl pallet_base_fee::Config for Runtime {
	//type RuntimeEvent = RuntimeEvent;
	type Threshold = BaseFeeThreshold;
	type DefaultBaseFeePerGas = DefaultBaseFeePerGas;
	type DefaultElasticity = DefaultElasticity;
}