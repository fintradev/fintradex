use crate::{
    constants::{currency::*, time::*},
    AccountId, Balance, BlockNumber, Preimage, Runtime, RuntimeCall, RuntimeEvent, RuntimeOrigin,
    Scheduler,
};
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::vec;
use frame_support::{instances::Instance2, parameter_types, traits::ConstU32};
use frame_system::{EnsureRoot, EnsureSigned, Pallet as System};
use sp_runtime::Perbill;

parameter_types! {
    pub const AlarmInterval: BlockNumber = 1;
    pub const SubmissionDeposit: Balance = 100 * DOLLARS;
    pub const UndecidingTimeout: BlockNumber = 28 * DAYS;
}

pub struct TracksInfo;
impl pallet_referenda::TracksInfo<Balance, BlockNumber> for TracksInfo {
    type Id = u16;
    type RuntimeOrigin = <RuntimeOrigin as frame_support::traits::OriginTrait>::PalletsOrigin;

    fn tracks(
    ) -> impl Iterator<Item = Cow<'static, pallet_referenda::Track<Self::Id, Balance, BlockNumber>>>
    {
        let root = Cow::Owned(
            pallet_referenda::Track::<Self::Id, Balance, BlockNumber, 25> {
                id: 0,
                info: pallet_referenda::TrackInfo {
                    name: {
                        let mut name = [0u8; 25];
                        let raw = b"root";
                        name[..raw.len()].copy_from_slice(raw);
                        name
                    },
                    max_deciding: 1,
                    decision_deposit: 10 * DOLLARS,
                    prepare_period: 4 * DAYS,
                    decision_period: 4 * DAYS,
                    confirm_period: 2 * DAYS,
                    min_enactment_period: 4 * DAYS,
                    min_approval: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(50),
                        ceil: Perbill::from_percent(100),
                    },
                    min_support: pallet_referenda::Curve::LinearDecreasing {
                        length: Perbill::from_percent(100),
                        floor: Perbill::from_percent(0),
                        ceil: Perbill::from_percent(100),
                    },
                },
            },
        );
        Box::new(vec![root].into_iter())
            as Box<
                dyn Iterator<
                    Item = Cow<'static, pallet_referenda::Track<Self::Id, Balance, BlockNumber>>,
                >,
            >
    }

    fn track_for(id: &Self::RuntimeOrigin) -> Result<Self::Id, ()> {
        if let Ok(system_origin) = frame_system::RawOrigin::try_from(id.clone()) {
            match system_origin {
                frame_system::RawOrigin::Root => Ok(0),
                _ => Err(()),
            }
        } else {
            Err(())
        }
    }
}

impl pallet_referenda::Config for Runtime {
    type WeightInfo = pallet_referenda::weights::SubstrateWeight<Self>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Currency = pallet_balances::Pallet<Self>;
    type SubmitOrigin = EnsureSigned<AccountId>;
    type CancelOrigin = EnsureRoot<AccountId>;
    type KillOrigin = EnsureRoot<AccountId>;
    type Slash = ();
    type Votes = pallet_conviction_voting::VotesOf<Runtime>;
    type Tally = pallet_conviction_voting::TallyOf<Runtime>;
    type SubmissionDeposit = SubmissionDeposit;
    type MaxQueued = ConstU32<100>;
    type UndecidingTimeout = UndecidingTimeout;
    type AlarmInterval = AlarmInterval;
    type Tracks = TracksInfo;
    type Preimages = Preimage;
    type BlockNumberProvider = System<Runtime>;
}

impl pallet_referenda::Config<Instance2> for Runtime {
    type WeightInfo = pallet_referenda::weights::SubstrateWeight<Self>;
    type RuntimeCall = RuntimeCall;
    type RuntimeEvent = RuntimeEvent;
    type Scheduler = Scheduler;
    type Currency = pallet_balances::Pallet<Self>;
    type SubmitOrigin = EnsureSigned<AccountId>;
    type CancelOrigin = EnsureRoot<AccountId>;
    type KillOrigin = EnsureRoot<AccountId>;
    type Slash = ();
    type Votes = pallet_ranked_collective::Votes;
    type Tally = pallet_ranked_collective::TallyOf<Runtime>;
    type SubmissionDeposit = SubmissionDeposit;
    type MaxQueued = ConstU32<100>;
    type UndecidingTimeout = UndecidingTimeout;
    type AlarmInterval = AlarmInterval;
    type Tracks = TracksInfo;
    type Preimages = Preimage;
    type BlockNumberProvider = System<Runtime>;
}
