use crate::{
    constants::currency::*, AccountId, Balances, RankedCollective, Runtime, RuntimeEvent, Treasury,BlockNumber,DAYS
};
use frame_support::{
    parameter_types,
    traits::{
        tokens::{GetSalary, PayFromAccount},
        ConstU32,
    },
};
parameter_types! {
    pub const Budget: Balance = 10_000 * FINTS;
    pub TreasuryAccount: AccountId = Treasury::account_id();
    // Cadences (use your BlockNumber-based DAYS constant)
    pub const RegistrationPeriod: BlockNumber = 14 * DAYS; // join/changes window
    pub const PayoutPeriod: BlockNumber       = 30 * DAYS; // monthly payroll
    
}

pub struct SalaryForRank;
impl GetSalary<u16, AccountId, Balance> for SalaryForRank {
    /*fn get_salary(a: u16, _: &AccountId) -> Balance {
        Balance::from(a) * 1000 * FINTS
    }*/
    fn get_salary(rank: u16, _who: &AccountId) -> Balance {
        use sp_runtime::traits::{Saturating, Bounded};
        let base   : Balance = 5 * FINTS;
        let per_rank: Balance = 2 * FINTS;
        let cap    : Balance = 100 * FINTS;
        let raw = base.saturating_add(per_rank.saturating_mul(rank as Balance));
        raw.min(cap)
    }
}
impl pallet_salary::Config for Runtime {
    type WeightInfo = pallet_salary::weights::SubstrateWeight<Self>;
    type RuntimeEvent = RuntimeEvent;
    type Paymaster = PayFromAccount<Balances, TreasuryAccount>;
    type Members = RankedCollective;
    type Salary = SalaryForRank;
    type RegistrationPeriod = RegistrationPeriod;
    type PayoutPeriod = PayoutPeriod;
    type Budget = Budget;
}
