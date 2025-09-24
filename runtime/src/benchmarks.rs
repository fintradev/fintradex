
//! # FintradeX Runtime Benchmarks
//!
//! This module contains runtime benchmarks for the FintradeX parachain, providing
//! performance measurements and weight calculations for all pallets and operations.
//!
//! ## Benchmark Coverage
//!
//! - **Trading Operations**: Order placement, matching, and execution
//! - **Asset Management**: Cross-chain asset transfers and conversions
//! - **Governance**: Democratic voting and proposal execution
//! - **EVM Operations**: Smart contract execution and gas calculations
//!
//! ## Performance Metrics
//!
//! - Execution time measurements
//! - Memory usage analysis
//! - Gas cost calculations
//! - Weight optimization recommendations
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

polkadot_sdk::frame_benchmarking::define_benchmarks!(
	[frame_system, SystemBench::<Runtime>]
	[pallet_balances, Balances]
	[pallet_session, SessionBench::<Runtime>]
	[pallet_timestamp, Timestamp]
	[pallet_message_queue, MessageQueue]
	[pallet_sudo, Sudo]
	[pallet_collator_selection, CollatorSelection]
	[pallet_bags_list, VoterList]
	[pallet_balances, Balances]
	[pallet_bounties, Bounties]
	[pallet_broker, Broker]
	[pallet_democracy, Democracy]
	[pallet_child_bounties, ChildBounties]
	[pallet_collective, Council]
	[pallet_conviction_voting, ConvictionVoting]
	[pallet_contracts, Contracts]
	[pallet_core_fellowship, CoreFellowship]
	[pallet_asset_conversion, AssetConversion]
	[pallet_election_provider_multi_phase, ElectionProviderMultiPhase]
	[pallet_fast_unstake, FastUnstake]
	[pallet_parameters, Parameters]
	[pallet_indices, Indices]
	[pallet_membership, TechnicalMembership]
	[pallet_staking, Staking]
	[pallet_treasury, Treasury]
	[pallet_asset_rate, AssetRate]
	[pallet_vesting, Vesting]
	[pallet_whitelist, Whitelist]
	[pallet_session, SessionBench::<Runtime>]
	[pallet_multisig, Multisig]
	[pallet_nomination_pools, NominationPoolsBench::<Runtime>]
	[pallet_preimage, Preimage]
	[pallet_referenda, Referenda]
	[pallet_salary, Salary]
	[pallet_scheduler, Scheduler]
	[pallet_asset_conversion_ops, AssetConversionMigration]
	[pallet_evm, EVM]
	[cumulus_pallet_parachain_system, ParachainSystem]
	[cumulus_pallet_xcmp_queue, XcmpQueue]
);
