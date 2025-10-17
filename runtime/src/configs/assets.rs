//! Token Pallet Configuration
//!
//! This module defines the configuration for the token pallet, which handles
//! the native Fintra token functionality.

use crate::{
    configs::asset_conversion::AssetConversionPalletId, constants::currency::*, AccountId,
    Balances, Runtime, RuntimeEvent,
};
use frame_support::{
    instances::{Instance1, Instance2},
    ord_parameter_types, parameter_types,
    traits::{AsEnsureOriginWithArg, ConstU128, ConstU32},
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use scale_info::prelude::vec;
use sp_runtime::traits::AccountIdConversion;
use sp_runtime::AccountId32;
parameter_types! {
    pub const AssetDeposit: Balance = 100 * DOLLARS;
    pub const ApprovalDeposit: Balance = DOLLARS;
    pub const StringLimit: u32 = 50;
    pub const MetadataDepositBase: Balance = 10 * DOLLARS;
    pub const MetadataDepositPerByte: Balance = DOLLARS;
}
impl pallet_assets::Config<Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = <Runtime as pallet_balances::Config>::Balance;
    type AssetId = u32;
    type Currency = Balances;
    type AssetIdParameter = codec::Compact<u32>;
    type Holder = ();
    type CreateOrigin =
        frame_support::traits::AsEnsureOriginWithArg<frame_system::EnsureSigned<AccountId32>>;
    type ForceOrigin = frame_system::EnsureRoot<u64>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<DOLLARS>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = frame_support::traits::ConstU32<1000>;
    type CallbackHandle = ();
}
ord_parameter_types! {
    pub const AssetConversionOrigin: AccountId = AccountIdConversion::<AccountId>::into_account_truncating(&AssetConversionPalletId::get());
}
impl pallet_assets::Config<Instance2> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = u128;
    type AssetId = u32;
    type AssetIdParameter = codec::Compact<u32>;
    type Currency = Balances;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, AccountId>>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<DOLLARS>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = ConstU32<1000>;
    type CallbackHandle = ();
    type Holder = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}
