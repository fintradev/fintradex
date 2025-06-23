#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "1024"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod apis;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarks;
pub mod configs;
pub mod constants;
pub mod precompiles;
mod genesis_config_presets;
mod weights;
mod voter_bags;

extern crate alloc;
use core::marker::PhantomData;
use alloc::vec::Vec;
use smallvec::smallvec;
use constants::currency::*;

use polkadot_sdk::{staging_parachain_info as parachain_info, *};

use sp_runtime::{
	generic, impl_opaque_keys,
	traits::{BlakeTwo256, Block as BlockT, IdentifyAccount, Verify,NumberFor,DispatchInfoOf,PostDispatchInfoOf},
	transaction_validity::{TransactionPriority, TransactionSource, TransactionValidity,TransactionValidityError},
	ApplyExtrinsicResult, MultiSignature,MultiAddress
};
use sp_core::{crypto::KeyTypeId, OpaqueMetadata,H160,U256,H256,Get};
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
use codec::{Decode, Encode};
use pallet_session::historical as pallet_session_historical;
use frame_support::{weights::{
	constants::WEIGHT_REF_TIME_PER_SECOND, Weight, WeightToFeeCoefficient, WeightToFeeCoefficients,
	WeightToFeePolynomial
},BoundedVec,PalletId,StorageValue,};
use frame_support::{
	dynamic_params::{dynamic_pallet_params, dynamic_params},
	dispatch::DispatchClass,
	instances::{Instance1, Instance2},
	ord_parameter_types, parameter_types,
	traits::{AsEnsureOriginWithArg, ConstU128, ConstU32},
};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
pub use sp_runtime::{Perbill, Permill,traits::{Dispatchable,UniqueSaturatedInto},AccountId32};
pub use pallet_balances::Call as BalancesCall;
pub use weights::*;
//use weights::ExtrinsicBaseWeight;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Nonce = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// An index to a block.
pub type BlockNumber = u32;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;


/// The extension to the basic transaction logic.
#[docify::export(template_signed_extra)]
pub type TxExtension = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	cumulus_primitives_storage_weight_reclaim::StorageWeightReclaim<Runtime>,
	frame_metadata_hash_extension::CheckMetadataHash<Runtime>,
);
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
	cumulus_pallet_weight_reclaim::StorageWeightReclaim<Runtime, ()>,
);
/// Unchecked extrinsic type as expected by this runtime.
//pub type UncheckedExtrinsic =
	//generic::UncheckedExtrinsic<Address, RuntimeCall, Signature, TxExtension>;
pub type UncheckedExtrinsic =
	fp_self_contained::UncheckedExtrinsic<Address, RuntimeCall, Signature, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic =
	fp_self_contained::CheckedExtrinsic<AccountId, RuntimeCall, SignedExtra, H160>;
/// All migrations of the runtime, aside from the ones declared in the pallets.
///
/// This can be a tuple of types, each implementing `OnRuntimeUpgrade`.
#[allow(unused_parens)]
type Migrations = ();

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	AllPalletsWithSystem,
	Migrations,
>;

/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
/// node's balance type.
///
/// This should typically create a mapping between the following ranges:
///   - `[0, MAXIMUM_BLOCK_WEIGHT]`
///   - `[Balance::min, Balance::max]`
///
/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
///   - Setting it to `0` will essentially disable the weight fee.
///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		// in Rococo, extrinsic base weight (smallest non-zero weight) is mapped to 1 MILLI_UNIT:
		// in our template, we map to 1/10 of that, or 1/10 MILLI_UNIT
		let p = MILLI_UNIT / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

/// Opaque types. These are used by the CLI to instantiate machinery that don't need to know
/// the specifics of the runtime. They can then be made to be agnostic over specific formats
/// of data like extrinsics, allowing for them to continue syncing the network through upgrades
/// to even the core data structures.
pub mod opaque {
	use super::*;
	pub use polkadot_sdk::sp_runtime::OpaqueExtrinsic as UncheckedExtrinsic;
	use polkadot_sdk::sp_runtime::{
		generic,
		traits::{BlakeTwo256, Hash as HashT},
	};

	/// Opaque block header type.
	pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// Opaque block type.
	pub type Block = generic::Block<Header, UncheckedExtrinsic>;
	/// Opaque block identifier type.
	pub type BlockId = generic::BlockId<Block>;
	/// Opaque block hash type.
	pub type Hash = <BlakeTwo256 as HashT>::Output;
}

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}
#[dynamic_params(RuntimeParameters, pallet_parameters::Parameters::<Runtime>)]
pub mod dynamic_params {
	use super::*;

	#[dynamic_pallet_params]
	#[codec(index = 0)]
	pub mod storage {
		/// Configures the base deposit of storing some data.
		#[codec(index = 0)]
		pub static BaseDeposit: Balance = DOLLARS;

		/// Configures the per-byte deposit of storing some data.
		#[codec(index = 1)]
		pub static ByteDeposit: Balance = CENTS;
	}
}
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: alloc::borrow::Cow::Borrowed("fintradex-runtime"),
	impl_name: alloc::borrow::Cow::Borrowed("fintradex-runtime"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 0,
	apis: apis::RUNTIME_API_VERSIONS,
	transaction_version: 1,
	system_version: 1,
};
impl<B: BlockT> fp_rpc::ConvertTransaction<<B as BlockT>::Extrinsic> for TransactionConverter<B> {
	fn convert_transaction(
		&self,
		transaction: pallet_ethereum::Transaction,
	) -> <B as BlockT>::Extrinsic {
		let extrinsic = UncheckedExtrinsic::new_bare(
			pallet_ethereum::Call::<Runtime>::transact { transaction }.into(),
		);
		let encoded = extrinsic.encode();
		<B as BlockT>::Extrinsic::decode(&mut &encoded[..])
			.expect("Encoded extrinsic is always valid")
	}
}

impl fp_self_contained::SelfContainedCall for RuntimeCall {
	type SignedInfo = H160;

	fn is_self_contained(&self) -> bool {
		match self {
			RuntimeCall::Ethereum(call) => call.is_self_contained(),
			_ => false,
		}
	}

	fn check_self_contained(&self) -> Option<Result<Self::SignedInfo, TransactionValidityError>> {
		match self {
			RuntimeCall::Ethereum(call) => call.check_self_contained(),
			_ => None,
		}
	}

	fn validate_self_contained(
		&self,
		info: &Self::SignedInfo,
		dispatch_info: &DispatchInfoOf<RuntimeCall>,
		len: usize,
	) -> Option<TransactionValidity> {
		match self {
			RuntimeCall::Ethereum(call) => call.validate_self_contained(info, dispatch_info, len),
			_ => None,
		}
	}

	fn pre_dispatch_self_contained(
		&self,
		info: &Self::SignedInfo,
		dispatch_info: &DispatchInfoOf<RuntimeCall>,
		len: usize,
	) -> Option<Result<(), TransactionValidityError>> {
		match self {
			RuntimeCall::Ethereum(call) => {
				call.pre_dispatch_self_contained(info, dispatch_info, len)
			}
			_ => None,
		}
	}

	fn apply_self_contained(
		self,
		info: Self::SignedInfo,
	) -> Option<sp_runtime::DispatchResultWithInfo<PostDispatchInfoOf<Self>>> {
		match self {
			call @ RuntimeCall::Ethereum(pallet_ethereum::Call::transact { .. }) => {
				Some(call.dispatch(RuntimeOrigin::from(
					pallet_ethereum::RawOrigin::EthereumTransaction(info),
				)))
			}
			_ => None,
		}
	}
}
#[docify::export]
mod block_times {
	/// This determines the average expected block time that we are targeting. Blocks will be
	/// produced at a minimum duration defined by `SLOT_DURATION`. `SLOT_DURATION` is picked up by
	/// `pallet_timestamp` which is in turn picked up by `pallet_aura` to implement `fn
	/// slot_duration()`.
	///
	/// Change this to adjust the block time.
	pub const MILLI_SECS_PER_BLOCK: u64 = 6000;

	// NOTE: Currently it is not possible to change the slot duration after the chain has started.
	// Attempting to do so will brick block production.
	pub const SLOT_DURATION: u64 = MILLI_SECS_PER_BLOCK;
}
pub use block_times::*;

// Time is measured by number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLI_SECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

// Unit = the base number of indivisible units for balances
pub const UNIT: Balance = 1_000_000_000_000;
pub const MILLI_UNIT: Balance = 1_000_000_000;
pub const MICRO_UNIT: Balance = 1_000_000;
parameter_types! {
pub MaxCollectivesProposalWeight: Weight = Perbill::from_percent(50) * RuntimeBlockWeights::get().max_block;
pub const ImOnlineUnsignedPriority: TransactionPriority = TransactionPriority::max_value();

	/// We prioritize im-online heartbeats over election solution submission.
	pub const StakingUnsignedPriority: TransactionPriority = TransactionPriority::max_value() / 2;
pub const MaxAuthorities: u32 = 100;
pub const MaxKeys: u32 = 10_000;
pub const MaxPeerInHeartbeats: u32 = 10_000;
pub BlockWeights: frame_system::limits::BlockWeights =
		frame_system::limits::BlockWeights::with_sensible_defaults(
			Weight::from_parts(2u64 * WEIGHT_REF_TIME_PER_SECOND, u64::MAX),
			NORMAL_DISPATCH_RATIO,
		);
	pub BlockLength: frame_system::limits::BlockLength = frame_system::limits::BlockLength
		::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockLength: frame_system::limits::BlockLength =
	frame_system::limits::BlockLength::max_with_normal_ratio(5 * 1024 * 1024, NORMAL_DISPATCH_RATIO);
	pub RuntimeBlockWeights: frame_system::limits::BlockWeights = frame_system::limits::BlockWeights::builder()
		.base_block(BlockExecutionWeight::get())
		.for_class(DispatchClass::all(), |weights| {
			weights.base_extrinsic = ExtrinsicBaseWeight::get();
		})
		.for_class(DispatchClass::Normal, |weights| {
			weights.max_total = Some(NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT);
		})
		.for_class(DispatchClass::Operational, |weights| {
			weights.max_total = Some(MAXIMUM_BLOCK_WEIGHT);
			// Operational transactions have some extra reserved space, so that they
			// are included even if block reached `MAXIMUM_BLOCK_WEIGHT`.
			weights.reserved = Some(
				MAXIMUM_BLOCK_WEIGHT - NORMAL_DISPATCH_RATIO * MAXIMUM_BLOCK_WEIGHT
			);
		})
		.avg_block_initialization(AVERAGE_ON_INITIALIZE_RATIO)
		.build_or_panic();
}
/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLI_UNIT;

/// We assume that ~5% of the block weight is consumed by `on_initialize` handlers. This is
/// used to limit the maximal weight of a single extrinsic.
const AVERAGE_ON_INITIALIZE_RATIO: Perbill = Perbill::from_percent(5);

/// We allow `Normal` extrinsics to fill up the block up to 75%, the rest can be used by
/// `Operational` extrinsics.
const NORMAL_DISPATCH_RATIO: Perbill = Perbill::from_percent(75);

#[docify::export(max_block_weight)]
/// We allow for 2 seconds of compute with a 6 second average block time.
const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
	WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2),
	cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
);

#[docify::export]
mod async_backing_params {
	/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
	/// into the relay chain.
	pub(crate) const UNINCLUDED_SEGMENT_CAPACITY: u32 = 3;
	/// How many parachain blocks are processed by the relay chain per parent. Limits the
	/// number of blocks authored per slot.
	pub(crate) const BLOCK_PROCESSING_VELOCITY: u32 = 1;
	/// Relay chain slot duration, in milliseconds.
	pub(crate) const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;
}
pub(crate) use async_backing_params::*;

#[docify::export]
/// Aura consensus hook
type ConsensusHook = cumulus_pallet_aura_ext::FixedVelocityConsensusHook<
	Runtime,
	RELAY_CHAIN_SLOT_DURATION_MILLIS,
	BLOCK_PROCESSING_VELOCITY,
	UNINCLUDED_SEGMENT_CAPACITY,
>;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

// Create the runtime by composing the FRAME pallets that were previously configured.
#[frame_support::runtime]
mod runtime {
	#[runtime::runtime]
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeFreezeReason,
		RuntimeHoldReason,
		RuntimeSlashReason,
		RuntimeLockId,
		RuntimeTask,RuntimeViewFunction
	)]
	pub struct Runtime;

	#[runtime::pallet_index(0)]
	pub type System = frame_system;
	#[runtime::pallet_index(1)]
	pub type ParachainSystem = cumulus_pallet_parachain_system;
	#[runtime::pallet_index(2)]
	pub type Utility = pallet_utility::Pallet<Runtime>;
	#[runtime::pallet_index(3)]
	pub type Timestamp = pallet_timestamp;
	#[runtime::pallet_index(4)]
	pub type ParachainInfo = parachain_info;
	#[runtime::pallet_index(5)]
	pub type Indices = pallet_indices::Pallet<Runtime>;
	// Monetary stuff.
	#[runtime::pallet_index(6)]
	pub type Balances = pallet_balances;
	#[runtime::pallet_index(7)]
	pub type Assets = pallet_assets::Pallet<Runtime, Instance1>;
	#[runtime::pallet_index(8)]
	pub type PoolAssets = pallet_assets::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(9)]
	pub type Salary = pallet_salary::Pallet<Runtime>;

	#[runtime::pallet_index(10)]
	pub type CoreFellowship = pallet_core_fellowship::Pallet<Runtime>;

	#[runtime::pallet_index(11)]
	pub type VoterList = pallet_bags_list::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(12)]
	pub type ChildBounties = pallet_child_bounties::Pallet<Runtime>;
	#[runtime::pallet_index(13)]
	pub type Referenda = pallet_referenda::Pallet<Runtime>;
	#[runtime::pallet_index(14)]
	pub type TransactionPayment = pallet_transaction_payment;
	#[runtime::pallet_index(15)]
	pub type Bounties = pallet_bounties::Pallet<Runtime>;
	// Governance
	#[runtime::pallet_index(16)]
	pub type Sudo = pallet_sudo;

	#[runtime::pallet_index(17)]
	pub type ImOnline = pallet_im_online::Pallet<Runtime>;

	#[runtime::pallet_index(18)]
	pub type AssetConversion = pallet_asset_conversion::Pallet<Runtime>;
	#[runtime::pallet_index(19)]
	pub type AssetRate = pallet_asset_rate::Pallet<Runtime>;
	
	// Collator support. The order of these 4 are important and shall not change.
	#[runtime::pallet_index(20)]
	pub type Authorship = pallet_authorship;
	#[runtime::pallet_index(21)]
	pub type CollatorSelection = pallet_collator_selection;
	#[runtime::pallet_index(22)]
	pub type RankedPolls = pallet_referenda::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(23)]
	pub type RankedCollective = pallet_ranked_collective::Pallet<Runtime>;

	#[runtime::pallet_index(24)]
	pub type FastUnstake = pallet_fast_unstake::Pallet<Runtime>;

	#[runtime::pallet_index(25)]
	pub type Multisig = pallet_multisig::Pallet<Runtime>;

	#[runtime::pallet_index(26)]
	pub type Vesting = pallet_vesting::Pallet<Runtime>;

	#[runtime::pallet_index(27)]
	pub type ElectionProviderMultiPhase = pallet_election_provider_multi_phase::Pallet<Runtime>;

	#[runtime::pallet_index(28)]
	pub type Staking = pallet_staking::Pallet<Runtime>;
	#[runtime::pallet_index(29)]
	pub type Session = pallet_session;
	#[runtime::pallet_index(30)]
	pub type Council = pallet_collective::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(31)]
	pub type TechnicalMembership = pallet_membership::Pallet<Runtime, Instance1>;

	#[runtime::pallet_index(32)]
	pub type TechnicalCommittee = pallet_collective::Pallet<Runtime, Instance2>;

	#[runtime::pallet_index(33)]
	pub type Preimage = pallet_preimage::Pallet<Runtime>;

	#[runtime::pallet_index(34)]
	pub type Treasury = pallet_treasury::Pallet<Runtime>;

	#[runtime::pallet_index(35)]
	pub type Contracts = pallet_contracts::Pallet<Runtime>;
	#[runtime::pallet_index(36)]
	pub type Aura = pallet_aura;
	#[runtime::pallet_index(37)]
	pub type AuraExt = cumulus_pallet_aura_ext;

	// XCM helpers.
	#[runtime::pallet_index(38)]
	pub type XcmpQueue = cumulus_pallet_xcmp_queue;
	#[runtime::pallet_index(39)]
	pub type PolkadotXcm = pallet_xcm;
	#[runtime::pallet_index(40)]
	pub type CumulusXcm = cumulus_pallet_xcm;
	#[runtime::pallet_index(41)]
	pub type MessageQueue = pallet_message_queue;
	#[runtime::pallet_index(42)]
	pub type Whitelist = pallet_whitelist::Pallet<Runtime>;

	#[runtime::pallet_index(43)]
	pub type Scheduler = pallet_scheduler::Pallet<Runtime>;

	#[runtime::pallet_index(44)]
	pub type ConvictionVoting = pallet_conviction_voting::Pallet<Runtime>;

	#[runtime::pallet_index(45)]
	pub type NominationPools = pallet_nomination_pools::Pallet<Runtime>;

	#[runtime::pallet_index(46)]
	pub type RandomnessCollectiveFlip = pallet_insecure_randomness_collective_flip::Pallet<Runtime>;
	#[runtime::pallet_index(47)]
	pub type Ethereum = pallet_ethereum::Pallet<Runtime>;

	#[runtime::pallet_index(48)]
	pub type EVM = pallet_evm::Pallet<Runtime>;

	#[runtime::pallet_index(49)]
	pub type EVMChainId = pallet_evm_chain_id::Pallet<Runtime>;

	#[runtime::pallet_index(50)]
	pub type BaseFee = pallet_base_fee::Pallet<Runtime>;

	#[runtime::pallet_index(51)]
	pub type Beefy = pallet_beefy::Pallet<Runtime>;

	#[runtime::pallet_index(52)]
	pub type Mmr = pallet_mmr::Pallet<Runtime>;

	#[runtime::pallet_index(53)]
	pub type MmrLeaf = pallet_beefy_mmr::Pallet<Runtime>;

	#[runtime::pallet_index(54)]
	pub type Offences = pallet_offences::Pallet<Runtime>;

	#[runtime::pallet_index(55)]
	pub type Historical = pallet_session_historical::Pallet<Runtime>;

	#[runtime::pallet_index(56)]
	pub type AssetConversionMigration = pallet_asset_conversion_ops::Pallet<Runtime>;

	#[runtime::pallet_index(57)]
	pub type Parameters = pallet_parameters::Pallet<Runtime>;

}

#[docify::export(register_validate_block)]
cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
}
#[derive(Clone)]
pub struct TransactionConverter<B>(PhantomData<B>);

impl<B> Default for TransactionConverter<B> {
	fn default() -> Self {
		Self(PhantomData)
	}
}
impl sp_core::Get<RuntimeVersion> for Runtime {
	fn get() -> RuntimeVersion {
		VERSION
	}
}