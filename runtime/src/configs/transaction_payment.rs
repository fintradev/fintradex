//! Transaction Payment Pallet Configuration
//! 
//! This module defines the configuration for the transaction payment pallet,
//! which handles transaction fees and payments.

use frame_support::{
    parameter_types,
    traits::ConstU8,
    weights::IdentityFee,
};
use pallet_transaction_payment::{ConstFeeMultiplier, FungibleAdapter, Multiplier};
use sp_runtime::traits::One;

use crate::{
    Balance, Runtime, RuntimeEvent, Balances,
};

parameter_types! {
    pub FeeMultiplier: Multiplier = Multiplier::one();
}

impl pallet_transaction_payment::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = FungibleAdapter<Balances, ()>;
    type OperationalFeeMultiplier = ConstU8<5>;
    type WeightToFee = IdentityFee<Balance>;
    type LengthToFee = IdentityFee<Balance>;
    type FeeMultiplierUpdate = ConstFeeMultiplier<FeeMultiplier>;
    type WeightInfo = pallet_transaction_payment::weights::SubstrateWeight<Runtime>;
} 