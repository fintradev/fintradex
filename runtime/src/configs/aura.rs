//! Aura Pallet Configuration
//! 
//! This module defines the configuration for the Aura pallet, which provides
//! block authoring and slot-based consensus.

use frame_support::traits::{ConstBool, ConstU32};
use sp_consensus_aura::sr25519::AuthorityId as AuraId;

use crate::{
    Runtime, Aura, SLOT_DURATION,
};

impl pallet_aura::Config for Runtime {
    type AuthorityId = AuraId;
    type DisabledValidators = ();
    type MaxAuthorities = ConstU32<32>;
    type AllowMultipleBlocksPerSlot = ConstBool<false>;
    type SlotDuration = pallet_aura::MinimumPeriodTimesTwo<Runtime>;
} 