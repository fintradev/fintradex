//! Token Pallet Configuration
//!
//! This module defines the configuration for the token pallet, which handles
//! the native Fintra token functionality.

use crate::{
    configs::asset_conversion::AssetConversionPalletId, constants::currency::*, AccountId,
    Balances, Runtime, RuntimeEvent,
};
use frame_support::{
    instances::{Instance1, Instance2},error::BadOrigin,
    ord_parameter_types, parameter_types,
    traits::{ConstU128, ConstU32,EnsureOriginWithArg,AsEnsureOriginWithArg},PalletId
};
use frame_system::{EnsureRoot,pallet_prelude::OriginFor,EnsureSignedBy};
use scale_info::prelude::vec;
use sp_runtime::traits::AccountIdConversion;
// Optional: a deterministic sentinel AccountId to return as Success.
// (pallet-assets ignores the Success value for create(); admin comes from call args.)
parameter_types! {
    pub const RootSentinelPid: PalletId = PalletId(*b"rtsntl__");
}
#[inline]
fn root_sentinel_account() -> AccountId {
    RootSentinelPid::get().into_account_truncating()
}

// --- Root-only origin that ignores the AssetId arg and returns an AccountId Success ---
pub struct EnsureRootWithAdminRuntime;

// Argument = AssetId (u32), Success = AccountId
impl EnsureOriginWithArg<OriginFor<Runtime>, u32> for EnsureRootWithAdminRuntime {
    type Success = AccountId;

    // try_origin must return Result<Success, OriginFor<Runtime>>
    fn try_origin(
        origin: OriginFor<Runtime>,
        _asset_id: &u32, // required by trait; not used
    ) -> Result<Self::Success, OriginFor<Runtime>> {
        // FRAME 41 EnsureRoot (WithArg form) needs a dummy arg: &()
        EnsureRoot::<AccountId>::try_origin(origin, &()).map(|_| root_sentinel_account())
    }

    // ensure_origin must return Result<Success, BadOrigin>
    fn ensure_origin(
        origin: OriginFor<Runtime>,
        _asset_id: &u32,
    ) -> Result<Self::Success, BadOrigin> {
        EnsureRoot::<AccountId>::ensure_origin(origin, &()).map(|_| root_sentinel_account())
    }
}
parameter_types! {
    pub const AssetDeposit: Balance = 10 * FINTS;
    pub const ApprovalDeposit: Balance = 1 * FINTS;
    pub const StringLimit: u32 = 128;
    pub const MetadataDepositBase: Balance = 1 * FINTS;
    pub const MetadataDepositPerByte: Balance = 1 * CENTI_FINTS;
    pub const AssetsAdminPalletId: PalletId = PalletId(*b"ft/admin");
}
ord_parameter_types! {
    pub const AssetsAdmin: AccountId = {
        // 👇 force the type so the macro stops guessing
        let acc: AccountId =
            <PalletId as AccountIdConversion<AccountId>>
                ::into_account_truncating(&AssetsAdminPalletId::get());
        acc
    };
    
}
type RuntimeBalance = <Runtime as pallet_balances::Config>::Balance;
impl pallet_assets::Config<Instance1> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = RuntimeBalance;
    type AssetId = u32;
    type Currency = Balances;
    type AssetIdParameter = codec::Compact<u32>;
    type Holder = ();
    //type CreateOrigin = EnsureRoot<AccountId>;
    //This is for later versions
    type CreateOrigin =EnsureRootWithAdminRuntime;
    //type CreateOrigin =
      // frame_system::EnsureSigned<AccountId>;
    //type ForceOrigin = frame_system::EnsureRoot<u64>;
    type ForceOrigin = EnsureRoot<AccountId>; 
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<FINTS>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = frame_support::traits::ConstU32<10_000>;
    type CallbackHandle = ();
}
ord_parameter_types! {
    pub const AssetConversionOrigin: AccountId = AccountIdConversion::<AccountId>::into_account_truncating(&AssetConversionPalletId::get());

}
impl pallet_assets::Config<Instance2> for Runtime {
    type RuntimeEvent = RuntimeEvent;
    //type Balance = u128;
    type Balance = RuntimeBalance;
    type AssetId = u32;
    type AssetIdParameter = codec::Compact<u32>;
    type Currency = Balances;
    //type CreateOrigin = EnsureRootWithAdminRuntime;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSignedBy<AssetConversionOrigin, AccountId>>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = ConstU128<{FINTS/10}>;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = StringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = pallet_assets::weights::SubstrateWeight<Runtime>;
    type RemoveItemsLimit = ConstU32<10_000>;
    type CallbackHandle = ();
    type Holder = ();
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}
