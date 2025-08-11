use crate::{
	constants::{currency::*, time::*},
	Balance, ChildBounties, Runtime, RuntimeEvent,Treasury,AccountId,Balances,Authorship,System
};
use frame_support::{parameter_types,traits::{
    fungible::{Balanced, Credit, HoldConsideration, ItemOf},
    tokens::{nonfungibles_v2::Inspect, pay::PayAssetFromAccount, GetSalary, PayFromAccount},
    AsEnsureOriginWithArg, ConstBool, ConstU128, ConstU16, ConstU32, Contains, Currency,
    EitherOfDiverse, EqualPrivilegeOnly, Imbalance, InsideBoth, InstanceFilter,
    KeyOwnerProofSystem, LinearStoragePrice, LockIdentifier, Nothing, OnUnbalanced,
    WithdrawReasons,
},
weights::{
    constants::{
        BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,
    },
    ConstantMultiplier, IdentityFee, Weight,
},
BoundedVec, PalletId,};
use sp_runtime::{Permill,traits::{Identity,Zero,MaybeConvert}};
use pallet_broker::{CoreAssignment, CoreIndex, CoretimeInterface, PartsOf57600};
use pallet_broker::TaskId;
use frame_system::EnsureRoot;
use alloc::vec::Vec;

parameter_types! {
	pub const BrokerPalletId: PalletId = PalletId(*b"py/broke");
	pub const MinimumCreditPurchase: Balance =  100 * MILLICENTS;
}

pub struct IntoAuthor;
impl OnUnbalanced<Credit<AccountId, Balances>> for IntoAuthor {
	fn on_nonzero_unbalanced(credit: Credit<AccountId, Balances>) {
		if let Some(author) = Authorship::author() {
			let _ = <Balances as Balanced<_>>::resolve(&author, credit);
		}
	}
}

pub struct CoretimeProvider;
impl CoretimeInterface for CoretimeProvider {
	type AccountId = AccountId;
	type Balance = Balance;
	type RelayChainBlockNumberProvider = System;
	fn request_core_count(_count: CoreIndex) {}
	fn request_revenue_info_at(_when: u32) {}
	fn credit_account(_who: Self::AccountId, _amount: Self::Balance) {}
	fn assign_core(
		_core: CoreIndex,
		_begin: u32,
		_assignment: Vec<(CoreAssignment, PartsOf57600)>,
		_end_hint: Option<u32>,
	) {
	}
}

pub struct SovereignAccountOf;
// Dummy implementation which converts `TaskId` to `AccountId`.
impl MaybeConvert<TaskId, AccountId> for SovereignAccountOf {
	fn maybe_convert(task: TaskId) -> Option<AccountId> {
		let mut account: [u8; 32] = [0; 32];
		account[..4].copy_from_slice(&task.to_le_bytes());
		Some(account.into())
	}
}
impl pallet_broker::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type OnRevenue = IntoAuthor;
	type TimeslicePeriod = ConstU32<2>;
	type MaxLeasedCores = ConstU32<5>;
	type MaxReservedCores = ConstU32<5>;
	type Coretime = CoretimeProvider;
	type ConvertBalance = Identity;
	type WeightInfo = ();
	type PalletId = BrokerPalletId;
	type AdminOrigin = EnsureRoot<AccountId>;
	type SovereignAccountOf = SovereignAccountOf;
	type MaxAutoRenewals = ConstU32<10>;
	type PriceAdapter = pallet_broker::CenterTargetPrice<Balance>;
	type MinimumCreditPurchase = MinimumCreditPurchase;
}