use crate::{
    Runtime, RuntimeEvent,Historical,Offences,MaxKeys,MaxPeerInHeartbeats,
    ImOnlineUnsignedPriority,UncheckedExtrinsic,RuntimeCall
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use crate::configs::session::{SessionDuration,Offset};
impl frame_system::offchain::CreateInherent<pallet_im_online::Call<Runtime>> for Runtime {
	fn create_inherent(
		call: pallet_im_online::Call<Runtime>
	) -> UncheckedExtrinsic {
		UncheckedExtrinsic::new_bare(RuntimeCall::from(call))
	}
}
// Add CreateTransactionBase implementation for pallet_im_online::Call<Runtime>
impl frame_system::offchain::CreateTransactionBase<pallet_im_online::Call<Runtime>> for Runtime {
	type Extrinsic = UncheckedExtrinsic;
	type RuntimeCall = pallet_im_online::Call<Runtime>;
}
impl pallet_im_online::Config for Runtime {
	type AuthorityId = ImOnlineId;
	type RuntimeEvent = RuntimeEvent;
	//type NextSessionRotation = Aura;
    type NextSessionRotation = pallet_session::PeriodicSessions<SessionDuration, Offset>;
	type ValidatorSet = Historical;
	type ReportUnresponsiveness = Offences;
	type UnsignedPriority = ImOnlineUnsignedPriority;
	type WeightInfo = pallet_im_online::weights::SubstrateWeight<Runtime>;
	type MaxKeys = MaxKeys;
	type MaxPeerInHeartbeats = MaxPeerInHeartbeats;
}