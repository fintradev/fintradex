// This is free and unencumbered software released into the public domain.
//
// Anyone is free to copy, modify, publish, use, compile, sell, or
// distribute this software, either in source code form or as a compiled
// binary, for any purpose, commercial or non-commercial, and by any
// means.
//
// In jurisdictions that recognize copyright laws, the author or authors
// of this software dedicate any and all copyright interest in the
// software to the public domain. We make this dedication for the benefit
// of the public at large and to the detriment of our heirs and
// successors. We intend this dedication to be an overt act of
// relinquishment in perpetuity of all present and future rights to this
// software under copyright law.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// For more information, please refer to <http://unlicense.org>

//! Runtime Configuration
//! 
//! This module contains all the configuration for the Fintra runtime.
//! Each pallet's configuration is separated into its own file for better organization.

pub mod aura;
pub mod grandpa;
pub mod sudo;
pub mod timestamp;
pub mod assets;
pub mod balances;
pub mod system;
pub mod transaction_payment;
pub mod vesting;
pub mod child_bounties;
pub mod bounties;
pub mod multisig;
pub mod utility;
pub mod indices;
pub mod asset_conversion;
pub mod preimage;
pub mod parameter;
pub mod whitelist;
pub mod collective;
pub mod voting;
pub mod scheduler;
pub mod refrenda;
pub mod asset_rate;
pub mod treasury;
pub mod salary;
pub mod ranked;
pub mod fellowship;
pub mod bag_list;
pub mod session;
pub mod election;
pub mod membership;
pub mod technical_collective;
pub mod staking;
pub mod nomination_pool;
pub mod contracts;
pub mod evm;
pub mod ethereum;
pub mod base_fee;
pub mod beefy;
pub mod mmr;
pub mod im_online;
pub use aura::*;
pub use grandpa::*;
pub use sudo::*;
pub use timestamp::*;
pub use assets::*;
pub use balances::*;
pub use system::*;
pub use transaction_payment::*;
pub use child_bounties::*;
pub use bounties::*;
pub use vesting::*;
pub use multisig::*;
pub use utility::*;
pub use indices::*;
pub use asset_conversion::*;
pub use preimage::*;
pub use parameter::*;
pub use whitelist::*;
pub use collective::*;
pub use voting::*;
pub use scheduler::*;
pub use refrenda::*;
pub use asset_rate::*;
pub use treasury::*;
pub use salary::*;
pub use ranked::*;
pub use fellowship::*;
pub use bag_list::*;
pub use session::*;
pub use election::*;
pub use membership::*;
pub use technical_collective::*;
pub use collective::*;
pub use staking::*;
pub use nomination_pool::*;
pub use contracts::*;
pub use evm::*;
pub use ethereum::*;
pub use base_fee::*;
pub use beefy::*;
pub use mmr::*;
pub use im_online::*;

// Common imports
use frame_support::{
	parameter_types,
	traits::{ConstBool, ConstU128, ConstU32, ConstU64, ConstU8},
	weights::{
		constants::{RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND},
		IdentityFee, Weight,
	},
};
use frame_system::limits::{BlockLength, BlockWeights};
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_runtime::{traits::One, Perbill};
use sp_version::RuntimeVersion;

// Local module imports
use super::{
	AccountId, Aura, Balance, Balances, Block, BlockNumber, Hash, Nonce, PalletInfo, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeFreezeReason, RuntimeHoldReason, RuntimeOrigin, RuntimeTask,
	System, EXISTENTIAL_DEPOSIT, SLOT_DURATION, VERSION,
};

// Configuration for normal dispatch ratio
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

parameter_types! {
	pub const BlockHashCount: BlockNumber = 2400;
	pub const Version: RuntimeVersion = VERSION;

	/// We allow for 2 seconds of compute with a 6 second average block time.
	/// This is optimized for Fintra's order book operations and trading requirements.
	pub RuntimeBlockWeights: BlockWeights = BlockWeights::with_sensible_defaults(
		Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
		NORMAL_DISPATCH_RATIO,
	);
	pub RuntimeBlockLength: BlockLength = BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub const SS58Prefix: u8 = 42;
}

parameter_types! {
	pub FeeMultiplier: Multiplier = Multiplier::one();
}
