use crate::{constants::currency::*, Balances, Runtime, RuntimeCall, RuntimeEvent};
use frame_support::{parameter_types, traits::ConstU32};
use frame_system::Pallet as System;

parameter_types! {
    /// Base deposit required for creating a multisig execution or storing a dispatch call.
    /// This covers the storage cost for:
    /// - One storage item with key size of 32 bytes
    /// - Value size of 4+4+16+32 bytes = 56 bytes
    pub const DepositBase: Balance = deposit(1, 88);

    /// Additional deposit required per unit threshold when creating a multisig execution.
    /// This covers the storage cost for adding 32 bytes more into a pre-existing storage value.
    pub const DepositFactor: Balance = deposit(0, 32);
}

/// Implementation of the multisig pallet configuration for the runtime.
/// This pallet allows multiple accounts to jointly approve and execute transactions.
impl pallet_multisig::Config for Runtime {
    /// The overarching event type for the runtime.
    type RuntimeEvent = RuntimeEvent;

    /// The overarching call type for the runtime.
    type RuntimeCall = RuntimeCall;

    /// The currency mechanism for handling deposits and payments.
    type Currency = Balances;

    /// The base amount of currency needed to reserve for creating a multisig execution
    /// or to store a dispatch call for later.
    type DepositBase = DepositBase;

    /// The amount of currency needed per unit threshold when creating a multisig execution.
    type DepositFactor = DepositFactor;

    /// The maximum number of signatories allowed in a multisig.
    /// Set to 100 to allow for large multisig groups while preventing excessive storage usage.
    type MaxSignatories = ConstU32<100>;

    /// Weight information for the extrinsics in this pallet.
    type WeightInfo = pallet_multisig::weights::SubstrateWeight<Runtime>;

    /// Provider for the current block number.
    /// Uses the system pallet to get the local block number.
    type BlockNumberProvider = System<Runtime>;
}
