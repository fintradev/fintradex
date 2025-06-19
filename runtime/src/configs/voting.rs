use crate::{
	constants::time::*, AccountId, Balances, BlockNumber, Referenda, Runtime, RuntimeEvent,
};
use frame_support::{parameter_types, traits::ConstU32};
parameter_types! {
	pub const VoteLockingPeriod: BlockNumber = 30 * DAYS;
}
use frame_system::Pallet as System;

impl pallet_conviction_voting::Config for Runtime {
	type WeightInfo = pallet_conviction_voting::weights::SubstrateWeight<Self>;
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type VoteLockingPeriod = VoteLockingPeriod;
	type MaxVotes = ConstU32<512>;
	type MaxTurnout = frame_support::traits::TotalIssuanceOf<Balances, AccountId>;
	type Polls = Referenda;
	type BlockNumberProvider = System<Runtime>;
	type VotingHooks = ();
}
