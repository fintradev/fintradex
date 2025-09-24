//! # FintradeX Bounties Configuration
//!
//! This module configures the bounties system for the FintradeX parachain,
//! enabling community-driven development funding and ecosystem growth.
//!
//! ## Features
//!
//! - **Community Funding**: Decentralized funding for development initiatives
//! - **Bounty Management**: Automated bounty creation and management
//! - **Curator System**: Community-curated bounty evaluation
//! - **Treasury Integration**: Seamless integration with treasury operations
//!
//! ## Configuration
//!
//! - Bounty creation and management parameters
//! - Curator deposit requirements
//! - Minimum bounty values
//! - Treasury integration settings
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{
	constants::{currency::*, time::*},
	Balance, ChildBounties, Runtime, RuntimeEvent,Treasury
};
use frame_support::parameter_types;
use sp_runtime::Permill;
parameter_types! {
	pub const BountyCuratorDeposit: Permill = Permill::from_percent(50);
	pub const BountyValueMinimum: Balance = 5 * DOLLARS;
	pub const BountyDepositBase: Balance = DOLLARS;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub const CuratorDepositMin: Balance = DOLLARS;
	pub const CuratorDepositMax: Balance = 100 * DOLLARS;
	pub const BountyDepositPayoutDelay: BlockNumber = DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 14 * DAYS;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const MaximumReasonLength: u32 = 300;
}

impl pallet_bounties::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type BountyValueMinimum = BountyValueMinimum;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = pallet_bounties::weights::SubstrateWeight<Runtime>;
	type ChildBountyManager = ChildBounties;
	type OnSlash = Treasury;
}
