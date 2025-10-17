use crate::{constants::currency::*, Balances, Runtime, RuntimeEvent};
use frame_support::parameter_types;
parameter_types! {
    pub const IndexDeposit: Balance = DOLLARS;
}

impl pallet_indices::Config for Runtime {
    type AccountIndex = u32;
    type Currency = Balances;
    type Deposit = IndexDeposit;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = pallet_indices::weights::SubstrateWeight<Runtime>;
}
