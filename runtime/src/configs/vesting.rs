/*use crate::{constants::currency::*, Balances, Runtime, RuntimeEvent};*/
use crate::constants::currency::DOLLARS;
use crate::{Balance, Balances, Runtime, RuntimeEvent};
use frame_support::{parameter_types, traits::WithdrawReasons};
use sp_runtime::traits::ConvertInto;
parameter_types! {
    pub const MinVestedTransfer: Balance = 100 * DOLLARS;
    pub UnvestedFundsAllowedWithdrawReasons: WithdrawReasons =
        WithdrawReasons::except(WithdrawReasons::TRANSFER | WithdrawReasons::RESERVE);
}
impl pallet_vesting::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type BlockNumberToBalance = ConvertInto;
    type BlockNumberProvider = frame_system::Pallet<Runtime>;
    type MinVestedTransfer = MinVestedTransfer;
    type WeightInfo = pallet_vesting::weights::SubstrateWeight<Runtime>;
    type UnvestedFundsAllowedWithdrawReasons = UnvestedFundsAllowedWithdrawReasons;
    // `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
    // highest number of schedules that encodes less than 2^10.
    const MAX_VESTING_SCHEDULES: u32 = 28;
}
