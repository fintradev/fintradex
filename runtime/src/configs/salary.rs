use crate::{
    constants::currency::*, AccountId, Balances, RankedCollective, Runtime, RuntimeEvent, Treasury,
};
use frame_support::{
    parameter_types,
    traits::{
        tokens::{GetSalary, PayFromAccount},
        ConstU32,
    },
};
parameter_types! {
    pub const Budget: Balance = 10_000 * DOLLARS;
    pub TreasuryAccount: AccountId = Treasury::account_id();
}

pub struct SalaryForRank;
impl GetSalary<u16, AccountId, Balance> for SalaryForRank {
    fn get_salary(a: u16, _: &AccountId) -> Balance {
        Balance::from(a) * 1000 * DOLLARS
    }
}
impl pallet_salary::Config for Runtime {
    type WeightInfo = ();
    type RuntimeEvent = RuntimeEvent;
    type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
    type Members = RankedCollective;
    type Salary = SalaryForRank;
    type RegistrationPeriod = ConstU32<200>;
    type PayoutPeriod = ConstU32<200>;
    type Budget = Budget;
}
