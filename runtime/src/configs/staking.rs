use crate::{
    AccountId,Balances,Balance,BlockNumber,Timestamp,ElectionProviderMultiPhase, 
    Runtime, RuntimeEvent,RuntimeFreezeReason,RuntimeHoldReason,Treasury,NominationPools,VoterList,
	Session,Staking,constants::{currency::*}
};
use crate::configs::collective::CouncilCollective;
use frame_support::{parameter_types,traits::{ConstU32,OnUnbalanced,ConstU128,EitherOfDiverse,Filter, fungible::Imbalance, Imbalance as _}};
use frame_election_provider_support::onchain;
use sp_runtime::curve::PiecewiseLinear;
use frame_system::EnsureRoot;
use pallet_balances::NegativeImbalance;
use pallet_staking::RewardDestination;

parameter_types! {
	pub const SessionsPerEra: sp_staking::SessionIndex = 6;
	pub const BondingDuration: sp_staking::EraIndex = 24 * 28;
	pub const SlashDeferDuration: sp_staking::EraIndex = 24 * 7; // 1/4 the bonding duration.
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
	pub const MaxNominators: u32 = 64;
	pub const MaxControllersInDeprecationBatch: u32 = 5900;
	pub const OffchainRepeat: BlockNumber = 5;
	pub const HistoryDepth: u32 = 84;
}

/// Upper limit on the number of NPOS nominations.
const MAX_QUOTA_NOMINATIONS: u32 = 16;

pub struct StakingBenchmarkingConfig;
impl pallet_staking::BenchmarkingConfig for StakingBenchmarkingConfig {
	type MaxNominators = ConstU32<1000>;
	type MaxValidators = ConstU32<1000>;
}

pub struct TreasuryRewardRedirect;

impl OnUnbalanced<Imbalance<u128, frame_support::traits::fungible::DecreaseIssuance<AccountId, pallet_balances::Pallet<Runtime>>, frame_support::traits::fungible::IncreaseIssuance<AccountId, pallet_balances::Pallet<Runtime>>>> for TreasuryRewardRedirect {
	fn on_unbalanced(amount: Imbalance<u128, frame_support::traits::fungible::DecreaseIssuance<AccountId, pallet_balances::Pallet<Runtime>>, frame_support::traits::fungible::IncreaseIssuance<AccountId, pallet_balances::Pallet<Runtime>>>) {
		let negative_imbalance = NegativeImbalance::<Runtime>::new(amount.peek());
		pallet_treasury::Pallet::<Runtime>::on_unbalanced(negative_imbalance);
	}
}
pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

// Configure the Staking pallet
impl pallet_staking::Config for Runtime {
	type Currency = Balances;
	type CurrencyBalance = Balance;
	type UnixTime = Timestamp;
	type CurrencyToVote = sp_staking::currency_to_vote::U128CurrencyToVote;
	type ElectionProvider = ElectionProviderMultiPhase;
	//type GenesisElectionProvider = Self::ElectionProvider;
	type GenesisElectionProvider = onchain::OnChainExecution<crate::configs::election::OnChainSeqPhragmen>;
	type NominationsQuota = pallet_staking::FixedNominationsQuota<MAX_QUOTA_NOMINATIONS>;
	type RuntimeEvent = RuntimeEvent;
	type Slash = (); // No specific logic for slashing in a solo chain
	type Reward = (); // No specific logic for rewards in a solo chain
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SessionInterface = Self;
	type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
	type NextNewSession = Session;
	type WeightInfo = pallet_staking::weights::SubstrateWeight<Runtime>;
	type MaxUnlockingChunks = ConstU32<32>;
	type HistoryDepth = HistoryDepth;
	type BenchmarkingConfig = StakingBenchmarkingConfig;
	type RewardRemainder = TreasuryRewardRedirect;
	type AdminOrigin = EitherOfDiverse<
		EnsureRoot<AccountId>,
		pallet_collective::EnsureProportionAtLeast<AccountId, CouncilCollective, 3, 4>,
	>;
	type MaxExposurePageSize = ConstU32<256>;
	type VoterList = VoterList;
	type MaxControllersInDeprecationBatch = MaxControllersInDeprecationBatch;
	type TargetList=pallet_staking::UseValidatorsMap<Self>;
	type EventListeners=NominationPools;
	type OldCurrency=Balances;
	type RuntimeHoldReason=RuntimeHoldReason;
	//type Filter=pallet_staking::Filter<Runtime>;
	type Filter=();
	//type DisablingStrategy=pallet_staking::UpToLimitDisablingStrategy;
}
impl pallet_fast_unstake::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ControlOrigin = frame_system::EnsureRoot<AccountId>;
	type BatchSize = ConstU32<64>;
	type Deposit = ConstU128<{ DOLLARS }>;
	type Currency = Balances;
	type Staking = Staking;
	type MaxErasToCheckPerBlock = ConstU32<1>;
	type WeightInfo = ();
}