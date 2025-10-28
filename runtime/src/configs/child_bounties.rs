use crate::{constants::currency::*, Balance, Runtime, RuntimeEvent};
use frame_support::{parameter_types, traits::ConstU32};
parameter_types! {
    pub const ChildBountyValueMinimum: Balance = 2 * FINTS;
}

impl pallet_child_bounties::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type MaxActiveChildBountyCount = ConstU32<10>;
    type ChildBountyValueMinimum = ChildBountyValueMinimum;
    type WeightInfo = pallet_child_bounties::weights::SubstrateWeight<Runtime>;
}
