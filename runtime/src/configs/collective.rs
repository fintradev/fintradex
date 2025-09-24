//! # FintradeX Collective Configuration
//!
//! This module configures the collective governance system for the FintradeX parachain,
//! enabling council-based decision making and protocol management.
//!
//! ## Features
//!
//! - **Council Governance**: Elected council for protocol decisions
//! - **Proposal System**: Council proposal creation and voting
//! - **Executive Authority**: Council execution of approved proposals
//! - **Technical Committee**: Technical expertise for protocol decisions
//!
//! ## Configuration
//!
//! - Council composition and election
//! - Proposal creation and approval
//! - Voting mechanisms and thresholds
//! - Executive authority settings
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

use crate::{
	constants::time::*, MaxCollectivesProposalWeight, Runtime, RuntimeCall, RuntimeEvent,
	RuntimeOrigin,
};
use frame_support::{instances::Instance1, parameter_types};
use frame_system::EnsureRoot;
parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 5 * DAYS;
	pub const CouncilMaxProposals: u32 = 100;
	pub const CouncilMaxMembers: u32 = 100;
}
pub type CouncilCollective = Instance1;
impl pallet_collective::Config<CouncilCollective> for Runtime {
	type RuntimeOrigin = RuntimeOrigin;
	type Proposal = RuntimeCall;
	type RuntimeEvent = RuntimeEvent;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = CouncilMaxProposals;
	type MaxMembers = CouncilMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = pallet_collective::weights::SubstrateWeight<Runtime>;
	type SetMembersOrigin = EnsureRoot<Self::AccountId>;
	type MaxProposalWeight = MaxCollectivesProposalWeight;
	type DisapproveOrigin = EnsureRoot<Self::AccountId>;
	type KillOrigin = EnsureRoot<Self::AccountId>;
	type Consideration = ();
}
