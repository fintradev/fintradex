use crate::{constants::currency::*, AccountId, RankedCollective, Runtime, RuntimeEvent};
use frame_support::traits::{ConstU16, ConstU32};
use frame_system::EnsureRootWithSuccess;
impl pallet_core_fellowship::Config for Runtime {
	type WeightInfo = ();
	type RuntimeEvent = RuntimeEvent;
	type Members = RankedCollective;
	type Balance = Balance;
	type ParamsOrigin = frame_system::EnsureRoot<AccountId>;
	type InductOrigin = pallet_core_fellowship::EnsureInducted<Runtime, (), 1>;
	type ApproveOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
	type EvidenceSize = ConstU32<16_384>;
	type MaxRank = ConstU32<9>;
	type FastPromoteOrigin = Self::PromoteOrigin;
}
