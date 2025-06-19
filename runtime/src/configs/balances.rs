//! Balances Pallet Configuration
//! 
//! This module defines the configuration for the balances pallet, which handles
//! the native token balances of accounts.

use frame_support::{
    parameter_types,
    traits::{ConstU128, ConstU32, VariantCountOf},
};
use crate::{
    AccountId, Balance, Runtime, RuntimeEvent, RuntimeFreezeReason, RuntimeHoldReason,
    System, EXISTENTIAL_DEPOSIT,
};

parameter_types! {
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
    pub const MaxHolds: u32 = 10;
    pub const MaxFreezes: u32 = 10;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = ConstU32<50>;
    type MaxReserves = ConstU32<50>;
    type ReserveIdentifier = [u8; 8];
    /// The type for recording an account's balance.
    type Balance = Balance;
    /// The ubiquitous event type.
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ConstU128<{ EXISTENTIAL_DEPOSIT }>;
    type AccountStore = System;
    type WeightInfo = pallet_balances::weights::SubstrateWeight<Runtime>;
    type FreezeIdentifier = RuntimeFreezeReason;
    type MaxFreezes = ConstU32<10>;
    type RuntimeHoldReason = RuntimeHoldReason;
    type RuntimeFreezeReason = RuntimeFreezeReason;
    type DoneSlashHandler = ();
} 