//! # FintradeX BEEFY Configuration
//!
//! This module configures the BEEFY (Bridge Efficiency Enabling Finality Yielder) protocol
//! for the FintradeX parachain, enabling efficient finality proofs for cross-chain operations.
//!
//! ## Features
//!
//! - **Efficient Finality**: Optimized finality proofs for cross-chain bridges
//! - **Cross-Chain Support**: Enhanced interoperability with other blockchains
//! - **Security**: Cryptographic guarantees for finality verification
//! - **Performance**: High-performance finality mechanisms
//!
//! ## Configuration
//!
//! - BEEFY authority configuration
//! - Finality proof parameters
//! - Cross-chain bridge integration
//! - Security and validation settings
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{Runtime,MaxAuthorities,MmrLeaf,Offences,Historical,constants::time::*,UncheckedExtrinsic,RuntimeCall};
use frame_support::{parameter_types,traits::{ConstU32, KeyOwnerProofSystem}};
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_core::crypto::KeyTypeId;
use crate::configs::staking::{BondingDuration,SessionsPerEra};
parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_SLOTS;
	pub const BeefySetIdSessionEntries: u32 = BondingDuration::get() * SessionsPerEra::get();
	pub const ReportLongevity: u64 =
	BondingDuration::get() as u64 * SessionsPerEra::get() as u64 * EpochDuration::get();
}
impl frame_system::offchain::CreateInherent<pallet_beefy::Call<Runtime>> for Runtime {
	fn create_inherent(
		call: pallet_beefy::Call<Runtime>
	) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_bare(RuntimeCall::from(call))
	}
	fn create_bare(
		call: pallet_beefy::Call<Runtime>
	) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_bare(RuntimeCall::from(call))
	}
}
// Add CreateTransactionBase implementation for pallet_beefy::Call<Runtime>
impl frame_system::offchain::CreateTransactionBase<pallet_beefy::Call<Runtime>> for Runtime {
	type Extrinsic = UncheckedExtrinsic;
	type RuntimeCall = pallet_beefy::Call<Runtime>;
}
impl pallet_beefy::Config for Runtime {
	type BeefyId = BeefyId;
	type MaxAuthorities = MaxAuthorities;
	type MaxNominators = ConstU32<0>;
	type MaxSetIdSessionEntries = BeefySetIdSessionEntries;
	type OnNewValidatorSet = MmrLeaf;
	type AncestryHelper = MmrLeaf;
	type WeightInfo = ();
	type KeyOwnerProof = <Historical as KeyOwnerProofSystem<(KeyTypeId, BeefyId)>>::Proof;
	type EquivocationReportSystem =
		pallet_beefy::EquivocationReportSystem<Self, Offences, Historical, ReportLongevity>;
}