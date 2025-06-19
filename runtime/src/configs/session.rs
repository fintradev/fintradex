use crate::{
    AccountId,Balance,BlockNumber, Runtime, RuntimeEvent,Staking,constants::time::*,
};
use sp_runtime::traits::OpaqueKeys;
use frame_support::parameter_types;

parameter_types! {
    pub const SessionDuration: BlockNumber = 5 * MINUTES;
    pub const Offset: BlockNumber = 0;
}
impl pallet_session::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	type ValidatorIdOf = pallet_staking::StashOf<Self>;
	//type ShouldEndSession = Aura;
	//type NextSessionRotation = Aura;
    type ShouldEndSession = pallet_session::PeriodicSessions<SessionDuration, Offset>;
	type NextSessionRotation = pallet_session::PeriodicSessions<SessionDuration, Offset>;
	type SessionManager = pallet_session::historical::NoteHistoricalRoot<Self, Staking>;
	type SessionHandler = <crate::opaque::SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = crate::opaque::SessionKeys;
	type WeightInfo = pallet_session::weights::SubstrateWeight<Runtime>;
    //type DisablingStrategy = pallet_session::disabling::DisablingStrategy;
	type DisablingStrategy = ();
}
impl pallet_session::historical::Config for Runtime {
	type FullIdentification = pallet_staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = pallet_staking::ExposureOf<Runtime>;
}