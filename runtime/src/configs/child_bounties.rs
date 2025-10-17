use crate::{constants::currency::*, Balance, Runtime, RuntimeEvent};
use frame_support::{parameter_types, traits::ConstU32};
parameter_types! {
    pub const ChildBountyValueMinimum: Balance = DOLLARS;
}

impl pallet_child_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxActiveChildBountyCount = ConstU32<5>;
    type ChildBountyValueMinimum = ChildBountyValueMinimum;
    type WeightInfo = pallet_child_bounties::weights::SubstrateWeight<Runtime>;
}
