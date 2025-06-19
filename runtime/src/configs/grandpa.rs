//! Grandpa Pallet Configuration
//! 
//! This module defines the configuration for the Grandpa pallet, which provides
//! finality gadget for the blockchain.

use frame_support::traits::{ConstU32, ConstU64};
use sp_core::Void;

use crate::{
    Runtime, RuntimeEvent,
};

impl pallet_grandpa::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
    type MaxAuthorities = ConstU32<32>;
    type MaxNominators = ConstU32<0>;
    type MaxSetIdSessionEntries = ConstU64<0>;
    type KeyOwnerProof = Void;
    type EquivocationReportSystem = ();
} 