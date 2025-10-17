//! # FintraDex Scheduler Configuration
//!
//! This module configures the scheduler system for the FintraDex parachain,
//! enabling delayed execution of transactions and automated operations.
//!
//! ## Features
//!
//! - **Scheduled Execution**: Delayed transaction execution
//! - **Automated Operations**: Recurring and automated tasks
//! - **Governance Integration**: Scheduled governance operations
//! - **Trading Automation**: Automated trading and DeFi operations
//!
//! ## Configuration
//!
//! - Scheduling parameters and limits
//! - Execution timing and constraints
//! - Weight management for scheduled tasks
//! - Integration with other pallets
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{
    AccountId, OriginCaller, Perbill, Preimage, Runtime, RuntimeBlockWeights, RuntimeCall,
    RuntimeEvent, RuntimeOrigin, Weight,
};
use frame_support::{
    parameter_types,
    traits::{ConstU32, EqualPrivilegeOnly},
};
use frame_system::EnsureRoot;
parameter_types! {
    pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
        RuntimeBlockWeights::get().max_block;
}
use frame_system::Pallet as System;
impl pallet_scheduler::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type RuntimeOrigin = RuntimeOrigin;
    type PalletsOrigin = OriginCaller;
    type RuntimeCall = RuntimeCall;
    type MaximumWeight = MaximumSchedulerWeight;
    type ScheduleOrigin = EnsureRoot<AccountId>;
    #[cfg(feature = "runtime-benchmarks")]
    type MaxScheduledPerBlock = ConstU32<512>;
    #[cfg(not(feature = "runtime-benchmarks"))]
    type MaxScheduledPerBlock = ConstU32<50>;
    type WeightInfo = pallet_scheduler::weights::SubstrateWeight<Runtime>;
    type OriginPrivilegeCmp = EqualPrivilegeOnly;
    type Preimages = Preimage;
    type BlockNumberProvider = System<Runtime>;
}
