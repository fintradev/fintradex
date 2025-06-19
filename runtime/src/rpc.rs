//! Fintra-specific RPC methods.
//! 
//! This module extends the core RPC layer with Fintra-specific functionality:
//! - Token balance and transfer operations
//! - Order book queries and management
//! - Trading operations
//! - Asset management
//! - Market data access
//! - Cross-chain trading capabilities

#![warn(missing_docs)]

use std::sync::Arc;

use jsonrpsee::RpcModule;
use sc_transaction_pool_api::TransactionPool;
use fintra_runtime::{opaque::Block, AccountId, Balance, Nonce, TOKEN_NAME, TOKEN_SYMBOL, TOKEN_DECIMALS};
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// Full client dependencies for Fintra RPC.
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
}

/// Instantiate all full RPC extensions for Fintra.
pub fn create_full<C, P>(
	deps: FullDeps<C, P>,
) -> Result<RpcModule<()>, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>,
	C: HeaderBackend<Block> + HeaderMetadata<Block, Error = BlockChainError> + 'static,
	C: Send + Sync + 'static,
	C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + 'static,
{
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_frame_rpc_system::{System, SystemApiServer};

	let mut module = RpcModule::new(());
	let FullDeps { client, pool } = deps;

	module.merge(System::new(client.clone(), pool).into_rpc())?;
	module.merge(TransactionPayment::new(client).into_rpc())?;

	// Add Fintra token metadata RPC methods
	module.register_method("fintra_getTokenInfo", move |_| {
		Ok(serde_json::json!({
			"name": TOKEN_NAME,
			"symbol": TOKEN_SYMBOL,
			"decimals": TOKEN_DECIMALS,
		}))
	})?;

	// TODO: Add more Fintra-specific RPC methods here
	// - Token balance queries
	// - Transfer operations
	// - Order book queries
	// - Trading operations
	// - Asset management
	// - Market data access
	// - Cross-chain trading

	Ok(module)
} 