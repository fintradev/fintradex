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
	AccountId, AssetRate, Assets, Balance, Balances, BlockNumber, Bounties, Indices, PalletId,
	Runtime, RuntimeEvent, Treasury,
};
use frame_support::{
	parameter_types,
	traits::{tokens::pay::PayAssetFromAccount, EitherOfDiverse, ConstU32, ConstU16},
};
use frame_system::Pallet as System;
use frame_system::{EnsureRoot, EnsureWithSuccess};
use sp_runtime::{Percent, Permill};
use pallet_ranked_collective::EnsureMember;
parameter_types! {
	pub const SpendPeriod: BlockNumber = DAYS;
	pub const Burn: Permill = Permill::from_percent(50);
	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(20);
	pub const TipReportDepositBase: Balance = DOLLARS;
	pub const DataDepositPerByte: Balance = CENTS;
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	pub const MaximumReasonLength: u32 = 300;
	pub const MaxApprovals: u32 = 100;
	pub const MaxBalance: Balance = Balance::max_value();
	pub const SpendPayoutPeriod: BlockNumber = 30 * DAYS;
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = DOLLARS;
	pub TreasuryAccount: AccountId = Treasury::account_id();
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	/*type ApproveOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 5>,
	>;*/
//	type RejectOrigin = EitherOfDiverse<
//		EnsureRoot<AccountId>,
//		pallet_collective::EnsureProportionMoreThan<AccountId, CouncilCollective, 1, 2>,
//	>;
//type RejectOrigin = EnsureRoot<AccountId>;
	type RejectOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		// Allow treasury proposals to be rejected by ranked collective members (rank 5+)
		EnsureMember<Runtime, (), 5>
	>;
	type RuntimeEvent = RuntimeEvent;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	type SpendFunds = Bounties;
	type WeightInfo = pallet_treasury::weights::SubstrateWeight<Runtime>;
	type MaxApprovals = MaxApprovals;
	type SpendOrigin = EnsureWithSuccess<EnsureRoot<AccountId>, AccountId, MaxBalance>;
	type AssetKind = u32;
	type Beneficiary = AccountId;
	type BeneficiaryLookup = Indices;
	type Paymaster = PayAssetFromAccount<Assets, TreasuryAccount>;
	type BalanceConverter = AssetRate;
	type PayoutPeriod = SpendPayoutPeriod;
	type BlockNumberProvider = System<Runtime>;
	//type OnSlash = ();
	//type ProposalBond = ProposalBond;
	//type ProposalBondMinimum = ProposalBondMinimum;
	//type ProposalBondMaximum = ();
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}
