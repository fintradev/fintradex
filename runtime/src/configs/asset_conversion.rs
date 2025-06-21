use crate::configs::assets::AssetConversionOrigin;
use crate::{
	constants::currency::*, AccountId, Assets, Balance, Balances, PoolAssets, Runtime, RuntimeEvent,
};
use frame_support::{
	instances::Instance2,
	parameter_types,
	traits::{
		fungible::{NativeFromLeft, NativeOrWithId, UnionOf},
		tokens::imbalance::ResolveAssetTo,
		ConstU32,
	},
	PalletId,
};
use pallet_asset_conversion::{AccountIdConverter, Ascending, Chain, WithFirstAsset};
use sp_runtime::Permill;
parameter_types! {
	pub const AssetConversionPalletId: PalletId = PalletId(*b"py/ascon");
	pub const PoolSetupFee: Balance = DOLLARS; // should be more or equal to the existential deposit
	pub const MintMinLiquidity: Balance = 100;  // 100 is good enough when the main currency has 10-12 decimals.
	pub const LiquidityWithdrawalFee: Permill = Permill::from_percent(0);
	pub const Native: NativeOrWithId<u32> = NativeOrWithId::Native;
}

impl pallet_asset_conversion::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = u128;
	type HigherPrecisionBalance = primitive_types::U256;
	type AssetKind = NativeOrWithId<u32>;
	type Assets = UnionOf<Balances, Assets, NativeFromLeft, NativeOrWithId<u32>, AccountId>;
	type PoolId = (Self::AssetKind, Self::AssetKind);
	type PoolLocator = Chain<
		WithFirstAsset<
			Native,
			AccountId,
			NativeOrWithId<u32>,
			AccountIdConverter<AssetConversionPalletId, Self::PoolId>,
		>,
		Ascending<
			AccountId,
			NativeOrWithId<u32>,
			AccountIdConverter<AssetConversionPalletId, Self::PoolId>,
		>,
	>;
	type PoolAssetId = <Self as pallet_assets::Config<Instance2>>::AssetId;
	type PoolAssets = PoolAssets;
	type PoolSetupFee = PoolSetupFee;
	type PoolSetupFeeAsset = Native;
	type PoolSetupFeeTarget = ResolveAssetTo<AssetConversionOrigin, Self::Assets>;
	type PalletId = AssetConversionPalletId;
	type LPFee = ConstU32<3>; // means 0.3%
	type LiquidityWithdrawalFee = LiquidityWithdrawalFee;
	type WeightInfo = pallet_asset_conversion::weights::SubstrateWeight<Runtime>;
	type MaxSwapPathLength = ConstU32<4>;
	type MintMinLiquidity = MintMinLiquidity;
	#[cfg(feature = "runtime-benchmarks")]
	type BenchmarkHelper = ();
}
impl pallet_asset_conversion_ops::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type PriorAccountIdConverter = pallet_asset_conversion::AccountIdConverterNoSeed<(
		NativeOrWithId<u32>,
		NativeOrWithId<u32>,
	)>;
	type AssetsRefund = <Runtime as pallet_asset_conversion::Config>::Assets;
	type PoolAssetsRefund = <Runtime as pallet_asset_conversion::Config>::PoolAssets;
	type PoolAssetsTeam = <Runtime as pallet_asset_conversion::Config>::PoolAssets;
	type DepositAsset = Balances;
	type WeightInfo = pallet_asset_conversion_ops::weights::SubstrateWeight<Runtime>;
}