use pallet_ethereum::PostLogContent;
use crate::{Runtime, RuntimeEvent};
use crate::precompiles::FrontierPrecompiles;
pub use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{
		ConstBool, ConstU128, ConstU32, ConstU64, ConstU8, KeyOwnerProofSystem, Randomness,
		StorageInfo,FindAuthor,OnFinalize,Get
	},
	weights::{
		constants::{
			BlockExecutionWeight, ExtrinsicBaseWeight, RocksDbWeight, WEIGHT_REF_TIME_PER_SECOND,WEIGHT_REF_TIME_PER_MILLIS
		},
		IdentityFee, Weight,
	},
	StorageValue,
};
use frame_support::{
	genesis_builder_helper::{build_state, get_preset},
	traits::VariantCountOf,
};
parameter_types! {
	pub const PostBlockAndTxnHashes: PostLogContent = PostLogContent::BlockAndTxnHashes;
}

impl pallet_ethereum::Config for Runtime {
	//type RuntimeEvent = RuntimeEvent;
	type StateRoot = pallet_ethereum::IntermediateStateRoot<Self::Version>;
	type PostLogContent = PostBlockAndTxnHashes;
	type ExtraDataLength = ConstU32<30>;
}