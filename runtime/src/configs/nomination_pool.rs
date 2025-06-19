use crate::{Balances, Runtime,PalletId, RuntimeEvent,
	constants::currency::*,AccountId,RuntimeFreezeReason,Staking};
use crate::configs::collective::CouncilCollective;
use frame_support::{
	parameter_types,
	traits::{ConstU32,EitherOfDiverse}
};
use frame_system::EnsureRoot;
use frame_system::Pallet as System;
use sp_runtime::{
	traits::Convert,
	FixedU128,
};
parameter_types! {
	pub const PostUnbondPoolsWindow: u32 = 4;
	pub const NominationPoolsPalletId: PalletId = PalletId(*b"py/nopls");
	pub const MaxPointsToBalance: u8 = 10;
}
pub struct BalanceToU256;
impl Convert<Balance, primitive_types::U256> for BalanceToU256 {
	fn convert(balance: Balance) -> primitive_types::U256 {
		primitive_types::U256::from(balance)
	}
}
pub struct U256ToBalance;
impl Convert<primitive_types::U256, Balance> for U256ToBalance {
	fn convert(n: primitive_types::U256) -> Balance {
		n.try_into().unwrap_or(Balance::max_value())
	}
}

impl pallet_nomination_pools::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type RuntimeFreezeReason = RuntimeFreezeReason;
	type RewardCounter = FixedU128;
	type BalanceToU256 = BalanceToU256;
	type U256ToBalance = U256ToBalance;
	type StakeAdapter = pallet_nomination_pools::adapter::TransferStake<Self, Staking>;
	type PostUnbondingPoolsWindow = PostUnbondPoolsWindow;
	type MaxMetadataLen = ConstU32<256>;
	type MaxUnbonding = ConstU32<8>;
	type PalletId = NominationPoolsPalletId;
	type MaxPointsToBalance = MaxPointsToBalance;
	type BlockNumberProvider = System<Runtime>;
	type Filter = ();
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
}
