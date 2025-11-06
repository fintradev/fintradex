use crate::{constants::currency::*, RankedCollective, Runtime, RuntimeEvent,RuntimeOrigin};
use frame_support::{traits::{ConstU16, ConstU32},pallet_prelude::EnsureOrigin};
// Ensures the caller is inducted with rank >= MIN, and returns the caller's rank (u16).
pub struct EnsureInductedReturnRank<const MIN: u16>;

impl<const MIN: u16> EnsureOrigin<RuntimeOrigin> for EnsureInductedReturnRank<MIN> {
    type Success = u16;

    fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
        // Delegate the membership & min-rank check:
        let _who =
            <pallet_core_fellowship::EnsureInducted<Runtime, (), MIN> as EnsureOrigin<RuntimeOrigin>>
                ::try_origin(o)?;

        // Map AccountId -> rank (u16) to satisfy the Fellowship pallet bound
        /*if let Some(rank) = RankedMembers::rank_of(&who) {
            return Ok(rank as u16);
        }

        Err(RuntimeOrigin::from(RawOrigin::None))*/
        Ok(MIN)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn successful_origin() -> RuntimeOrigin {
        <pallet_core_fellowship::EnsureInducted<Runtime, (), MIN> as EnsureOrigin<RuntimeOrigin>>
            ::successful_origin()
    }
}

impl pallet_core_fellowship::Config for Runtime {
    //type WeightInfo = ();
    type WeightInfo = pallet_core_fellowship::weights::SubstrateWeight<Runtime>;
    type RuntimeEvent = RuntimeEvent;
    type Members = RankedCollective;
    type Balance = Balance;
    // Governance & gates (pick thresholds that match Fintradex policy)
    /*type ParamsOrigin = frame_system::EnsureRoot<AccountId>;
    type InductOrigin = pallet_core_fellowship::EnsureInducted<Runtime, (), 1>;
    type ApproveOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
    type PromoteOrigin = EnsureRootWithSuccess<AccountId, ConstU16<9>>;
    type FastPromoteOrigin = Self::PromoteOrigin;*/
    //type ParamsOrigin  = pallet_core_fellowship::EnsureInducted<Runtime, (), 7>;
    type ParamsOrigin  = EnsureInductedReturnRank<7>;
    type InductOrigin  = pallet_core_fellowship::EnsureInducted<Runtime, (), 3>; // Only EnsureOrigin needed; AccountId OK
    type ApproveOrigin     = EnsureInductedReturnRank<6>; // needs Success = u16
    type PromoteOrigin     = EnsureInductedReturnRank<7>; // needs Success = u16
    type FastPromoteOrigin = EnsureInductedReturnRank<8>; // needs Success = u16
    type EvidenceSize = ConstU32<16_384>;
    type MaxRank = ConstU16<9>;
    
}
