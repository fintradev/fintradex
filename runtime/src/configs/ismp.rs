use ismp::{
    Error,
	host::StateMachine,
	module::{IsmpModule},
	router::{IsmpRouter, PostRequest, Request, Response,Timeout}
};
use frame_support::parameter_types;
use frame_system::EnsureRoot;
use pallet_ismp::fee_handler::WeightFeeHandler;
use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::{
    AccountId,Balances,Balance,BlockNumber,Timestamp,ElectionProviderMultiPhase, 
    Runtime, RuntimeEvent,RuntimeFreezeReason,RuntimeHoldReason,Treasury,NominationPools,VoterList,
	Session,IsmpParachain,Ismp,constants::{currency::*},Weight
};



impl ismp_parachain::Config for Runtime {
	type IsmpHost = Ismp;
	type WeightInfo = IsmpWeights;
	type RootOrigin = EnsureRoot<AccountId>;
}
pub struct IsmpWeights;
impl ismp_parachain::weights::WeightInfo for IsmpWeights {
	fn add_parachain(_n: u32) -> Weight {
		Weight::from_parts(10_000, 0u64)
	}

	fn remove_parachain(_n: u32) -> Weight {
		Weight::from_parts(10_000, 0u64)
	}

	fn update_parachain_consensus() -> Weight {
		Weight::from_parts(10_000, 0u64)
	}
}
parameter_types! {
    // For example, the hyperbridge parachain on Polkadot
    pub const Coprocessor: Option<StateMachine> = Some(StateMachine::Polkadot(3367));
    // The host state machine of this pallet, your state machine id goes here
    pub const HostStateMachine: StateMachine = StateMachine::Polkadot(1000); // polkadot
    // pub const HostStateMachine: StateMachine = StateMachine::Kusama(1000); // kusama
    // pub const HostStateMachine: StateMachine = StateMachine::Substrate(*b"MYID"); // solochain
}
impl pallet_ismp::Config for Runtime {
    // Configure the runtime event
    //type RuntimeEvent = RuntimeEvent;
    // Permissioned origin who can create or update consensus clients
    type AdminOrigin = EnsureRoot<AccountId>;
    // The state machine identifier for this state machine
    type HostStateMachine = HostStateMachine;
    // The pallet_timestamp pallet
    type TimestampProvider = Timestamp;
    // The currency implementation that is offered to relayers
	// this could also be `frame_support::traits::tokens::fungible::ItemOf`
    type Currency = Balances;
    // The balance type for the currency implementation
    type Balance = Balance;
    // Router implementation for routing requests/responses to their respective modules
    type Router = Router;
    // Optional coprocessor for incoming requests/responses
    type Coprocessor = Coprocessor;
    // Supported consensus clients
    type ConsensusClients = (
        // as an example, the parachain consensus client
        ismp_parachain::ParachainConsensusClient<Runtime, IsmpParachain>,
    );
    // Offchain database implementation. Outgoing requests and responses are
    // inserted in this database, while their commitments are stored onchain.
    //
    // The default implementation for `()` should suffice
    type OffchainDB = ();
    // The fee handler implementation
    //type FeeHandler =WeightFeeHandler<()>;
    type FeeHandler = ();
}

#[derive(Default)]
pub struct Router;

impl IsmpRouter for Router {
	fn module_for_id(&self, _bytes: Vec<u8>) -> Result<Box<dyn IsmpModule>, anyhow::Error> {
		Ok(Box::new(ProxyModule::default()))
	}
}
#[derive(Default)]
pub struct ProxyModule;

impl IsmpModule for ProxyModule {
    fn on_accept(&self, request: PostRequest) -> Result<sp_runtime::Weight,anyhow::Error> {
        // do something useful with the request
        Ok(sp_runtime::Weight::from_parts(10_000, 0u64))
    }
 
    /// Called by the ISMP hanlder, to notify module of a response to a previously
    /// sent out request
    fn on_response(&self, response: Response) -> Result<sp_runtime::Weight, anyhow::Error> {
         // do something useful with the response
         Ok(sp_runtime::Weight::from_parts(10_000, 0u64))
    }
 
     /// Called by the ISMP hanlder, to notify module of requests that were previously
     /// sent but have now timed-out
 	fn on_timeout(&self, request: Timeout) -> Result<sp_runtime::Weight, anyhow::Error> {
        // revert any state changes that were made prior to dispatching the request
        Ok(sp_runtime::Weight::from_parts(10_000, 0u64))
    }
}