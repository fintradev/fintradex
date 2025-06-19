//! Sudo Pallet Configuration
//! 
//! This module defines the configuration for the Sudo pallet, which provides
//! superuser functionality for the blockchain.

use crate::{
    Runtime, RuntimeEvent,RuntimeCall,
};

impl pallet_sudo::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeCall = RuntimeCall;
    type WeightInfo = pallet_sudo::weights::SubstrateWeight<Runtime>;
} 