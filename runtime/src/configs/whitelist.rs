use crate::{AccountId, Preimage, Runtime, RuntimeCall, RuntimeEvent};
use frame_system::EnsureRoot;
impl pallet_whitelist::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type WhitelistOrigin = EnsureRoot<AccountId>;
	type DispatchWhitelistedOrigin = EnsureRoot<AccountId>;
	type Preimages = Preimage;
	type WeightInfo = pallet_whitelist::weights::SubstrateWeight<Runtime>;
}
