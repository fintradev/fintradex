//! # FintradeX Treasury Configuration
//!
//! This module configures the treasury system for the FintradeX parachain,
//! managing community funds and development resources.
//!
//! ## Features
//!
//! - **Community Treasury**: Decentralized fund management
//! - **Proposal System**: Community-driven funding proposals
//! - **Spend Management**: Transparent spending mechanisms
//! - **Asset Support**: Multi-asset treasury operations
//!
//! ## Configuration
//!
//! - Treasury parameters and limits
//! - Proposal creation and approval
//! - Spending mechanisms and controls
//! - Asset management settings
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

//use crate::configs::collective::CouncilCollective;
use crate::{
    constants::{currency::*, time::*},
    AccountId, AssetRate, Assets, Balance, Balances, BlockNumber, Bounties, PalletId,
    Runtime, RuntimeEvent, Treasury,
};
use frame_support::{
    parameter_types,
    traits::{tokens::pay::PayAssetFromAccount, EitherOfDiverse},
};
use frame_system::Pallet as System;
use frame_system::{EnsureRoot, EnsureRootWithSuccess};
use pallet_ranked_collective::EnsureMember;
use sp_runtime:: Permill;
parameter_types! {
    pub const SpendPeriod: BlockNumber = 7*DAYS;
    pub const Burn: Permill = Permill::from_percent(0);
    //pub const TipCountdown: BlockNumber = DAYS;
    //pub const TipFindersFee: Percent = Percent::from_percent(20);
    //pub const TipReportDepositBase: Balance = FINTS;
    pub const DataDepositPerByte: Balance = 1_000_000;
    pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
    pub const MaximumReasonLength: u32 = 300;
    pub const MaxApprovals: u32 = 100;
    pub const MaxBalance: Balance = Balance::max_value();
    pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
    pub const ProposalBond: Permill = Permill::from_percent(5);
    pub const ProposalBondMinimum: Balance = FINTS;
    pub TreasuryAccount: AccountId = Treasury::account_id();
}

impl pallet_treasury::Config for Runtime {
    type PalletId = TreasuryPalletId;
    type Currency = Balances;
    type RejectOrigin = EitherOfDiverse<
        EnsureRoot<AccountId>,
        // Allow treasury proposals to be rejected by ranked collective members (rank 5+)
        EnsureMember<Runtime, (), 5>,
    >;
    type RuntimeEvent = RuntimeEvent;
    type SpendPeriod = SpendPeriod;
    type Burn = Burn;
    type BurnDestination = ();
    type SpendFunds = (); // no routing; unspent stays in Treasury
    type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
    type MaxApprovals = MaxApprovals;
    type SpendOrigin = EnsureRootWithSuccess<AccountId, MaxBalance>;
    type AssetKind = u32;
    type Beneficiary = AccountId;
    //type BeneficiaryLookup = Indices;
    type BeneficiaryLookup = <Runtime as frame_system::Config>::Lookup;
    type Paymaster = PayAssetFromAccount<Assets, TreasuryAccount>;
    type BalanceConverter = AssetRate;
    type PayoutPeriod = SpendPayoutPeriod;
    type BlockNumberProvider = System<Runtime>;
    #[cfg(feature = "runtime-benchmarks")]
    type BenchmarkHelper = ();
}
