use crate::{AccountId, CoreFellowship, RankedPolls, Runtime, RuntimeEvent, Salary};
use frame_support::traits::ConstU16;
use frame_system::{EnsureRoot, EnsureRootWithSuccess};
use sp_runtime::traits::{Convert, Identity, MaybeConvert};

// Wrapper struct to implement MaybeConvert for MaxMemberCount
// This converts any u16 rank to a fixed u32 member count limit
pub struct ConstMaxMemberCount<const N: u32>;

impl<const N: u32> MaybeConvert<u16, u32> for ConstMaxMemberCount<N> {
	fn maybe_convert(_x: u16) -> Option<u32> {
		Some(N)
	}
}
pub struct MinRankOne<const N: u16>;
impl<const N: u16> Convert<u16, u16> for MinRankOne<N> {
    fn convert(_: u16) -> u16 {
        N
    }
}

impl pallet_ranked_collective::Config for Runtime {
    type WeightInfo = pallet_ranked_collective::weights::SubstrateWeight<Self>;
    type RuntimeEvent = RuntimeEvent;
    type AddOrigin = EnsureRoot<AccountId>;
    type RemoveOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type DemoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type ExchangeOrigin = EnsureRootWithSuccess<AccountId, ConstU16<65535>>;
    type Polls = RankedPolls;
    type MinRankOfClass = MinRankOne<1>;
    type VoteWeight = pallet_ranked_collective::Geometric;
    type MemberSwappedHandler = (CoreFellowship, Salary);
    type MaxMemberCount = ConstMaxMemberCount<64>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkSetup = (CoreFellowship, Salary);
}
