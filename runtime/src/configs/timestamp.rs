//! Timestamp Pallet Configuration
//! 
//! This module defines the configuration for the Timestamp pallet, which provides
//! time-related functionality for the blockchain.

use frame_support::traits::ConstU64;

use crate::{
    Runtime, RuntimeEvent, Aura, SLOT_DURATION,
};

impl pallet_timestamp::Config for Runtime {
    /// A timestamp: milliseconds since the unix epoch.
    type Moment = u64;
    type OnTimestampSet = Aura;
    type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
    type WeightInfo = ();
} 