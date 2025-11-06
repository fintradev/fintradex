//! # FintradeX Service Implementation
//!
//! This module provides the service and service factory implementation for the FintradeX parachain node.
//! It includes specialized wrappers over Substrate service with additional features for:
//!
//! - **Collator Service**: Parachain consensus and block production
//! - **EVM Integration**: Ethereum-compatible transaction processing
//! - **Cross-Chain Support**: Hyperbridge integration for interoperability
//! - **High-Performance Trading**: Optimized for trading operations
//!
//! ## Architecture
//!
//! The service handles:
//! - Block import and validation
//! - Transaction pool management
//! - RPC and WebSocket API endpoints
//! - Cross-chain message processing
//!
//! For more information, visit [https://fintradex.io/](https://fintradex.io/)

//! Service and ServiceFactory implementation. Specialized wrapper over substrate service.

use std::{sync::Arc, time::Duration};

use cumulus_client_cli::CollatorOptions;
// Local Runtime Types
use fintradex_runtime::{
    opaque::{Block, Hash},
    apis::RuntimeApi, TransactionConverter,
};

pub(crate) use crate::eth;
// Cumulus Imports
use cumulus_client_collator::service::CollatorService;
use cumulus_client_consensus_common::ParachainBlockImport as TParachainBlockImport;
use cumulus_client_consensus_proposer::Proposer;
use cumulus_client_service::{
    build_network, build_relay_chain_interface, prepare_node_config, start_relay_chain_tasks,
    BuildNetworkParams, CollatorSybilResistance, DARecoveryProfile, StartRelayChainTasksParams,
};
use cumulus_primitives_core::{relay_chain::CollatorPair, ParaId};
use cumulus_relay_chain_interface::{OverseerHandle, RelayChainInterface};

// Substrate Imports
use frame_benchmarking_cli::SUBSTRATE_REFERENCE_HARDWARE;
use sc_client_api::Backend;
use sc_consensus::ImportQueue;
use sc_executor::{
    HeapAllocStrategy, NativeElseWasmExecutor, WasmExecutor, DEFAULT_HEAP_ALLOC_STRATEGY,
};
use sc_network::NetworkBlock;
use sc_network::{config::FullNetworkConfiguration as NetCfg, service::NetworkWorker};
use sc_network_sync::SyncingService;
use sc_service::{Configuration, PartialComponents, TFullBackend, TFullClient, TaskManager};
use sc_telemetry::{Telemetry, TelemetryHandle, TelemetryWorker, TelemetryWorkerHandle};
use sc_transaction_pool_api::OffchainTransactionPoolFactory;
use sp_core::U256;
use sp_keystore::KeystorePtr;
pub use fc_rpc::StorageOverrideHandler;
pub use fc_storage::StorageOverride;
use polkadot_sdk::substrate_prometheus_endpoint::Registry;

// Frontier
use crate::eth::{new_frontier_partial, spawn_frontier_tasks, EthConfiguration,
    FrontierBackend, FrontierBlockImport as TFrontierBlockImport, FrontierPartialComponents,
};

/// Native executor type.
pub struct ParachainNativeExecutor;

impl sc_executor::NativeExecutionDispatch for ParachainNativeExecutor {
    type ExtendHostFunctions = frame_benchmarking::benchmarking::HostFunctions;

    fn dispatch(method: &str, data: &[u8]) -> Option<Vec<u8>> {
        fintradex_runtime::apis::dispatch(method, data)
    }

    fn native_version() -> sc_executor::NativeVersion {
        fintradex_runtime::native_version()
    }
}

type ParachainExecutor = NativeElseWasmExecutor<ParachainNativeExecutor>;

type ParachainClient = TFullClient<Block, RuntimeApi, ParachainExecutor>;

type ParachainBackend = TFullBackend<Block>;

type ParachainBlockImport = TParachainBlockImport<Block, FrontierBlockImport, ParachainBackend>;

type FrontierBlockImport = TFrontierBlockImport<Block, Arc<ParachainClient>, ParachainClient>;

/// Starts a `ServiceBuilder` for a full service.
///
/// Use this macro if you don't actually need the full service, but just the builder in order to
/// be able to perform chain operations.
#[allow(clippy::type_complexity)]
pub fn new_partial(
    config: &Configuration,
    eth_config: &EthConfiguration,
) -> Result<
    PartialComponents<
        ParachainClient,
        ParachainBackend,
        (),
        sc_consensus::DefaultImportQueue<Block>,
        Arc<sc_transaction_pool::TransactionPoolHandle<Block, ParachainClient>>,
        (
            ParachainBlockImport,
            Option<Telemetry>,
            Option<TelemetryWorkerHandle>,
            Arc<FrontierBackend<Block, ParachainClient>>,
            Arc<dyn StorageOverride<Block>>,
        ),
    >,
    sc_service::Error,
> {
    let telemetry = config
        .telemetry_endpoints
        .clone()
        .filter(|x| !x.is_empty())
        .map(|endpoints| -> Result<_, sc_telemetry::Error> {
            let worker = TelemetryWorker::new(16)?;
            let telemetry = worker.handle().new_telemetry(endpoints);
            Ok((worker, telemetry))
        })
        .transpose()?;

    let heap_pages = config.executor
        .default_heap_pages
        .map_or(DEFAULT_HEAP_ALLOC_STRATEGY, |h| HeapAllocStrategy::Static {
            extra_pages: h as _,
        });

    let wasm = WasmExecutor::builder()
        .with_execution_method(config.executor.wasm_method)
        .with_onchain_heap_alloc_strategy(heap_pages)
        .with_offchain_heap_alloc_strategy(heap_pages)
        .with_max_runtime_instances(config.executor.max_runtime_instances)
        .with_runtime_cache_size(config.executor.runtime_cache_size)
        .build();

    let executor = ParachainExecutor::new_with_wasm_executor(wasm);

    let (client, backend, keystore_container, task_manager) =
        sc_service::new_full_parts::<Block, RuntimeApi, _>(
            config,
            telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
            executor,
        )?;
    let client = Arc::new(client);

    let telemetry_worker_handle = telemetry.as_ref().map(|(worker, _)| worker.handle());

    let telemetry = telemetry.map(|(worker, telemetry)| {
        task_manager
            .spawn_handle()
            .spawn("telemetry", None, worker.run());
        telemetry
    });
    let transaction_pool = Arc::new(Arc::from(sc_transaction_pool::Builder::new(
			task_manager.spawn_essential_handle(),
			client.clone(),
			config.role.is_authority().into(),
		)
		.with_options(config.transaction_pool.clone())
		.with_prometheus(config.prometheus_registry())
		.build()));
    let overrides = Arc::new(StorageOverrideHandler::new(client.clone()));
        let frontier_backend = Arc::new(FrontierBackend::open(
            Arc::clone(&client),
            &config.database,
            &eth::db_config_dir(config),
        )?);
    //};

    let frontier_block_import = FrontierBlockImport::new(client.clone(), client.clone());

    let parachain_block_import = ParachainBlockImport::new(frontier_block_import, backend.clone());

    let import_queue = build_import_queue(
        client.clone(),
        parachain_block_import.clone(),
        config,
        eth_config,
        telemetry.as_ref().map(|telemetry| telemetry.handle()),
        &task_manager,
    )?;

    Ok(PartialComponents {
        backend,
        client,
        import_queue,
        keystore_container,
        task_manager,
        transaction_pool,
        select_chain: (),
        other: (
            parachain_block_import,
            telemetry,
            telemetry_worker_handle,
            frontier_backend,
            overrides,
        ),
    })
}

/// Start a node with the given parachain `Configuration` and relay chain `Configuration`.
///
/// This is the actual implementation that is abstract over the executor and the runtime api.
#[sc_tracing::logging::prefix_logs_with("Parachain")]
async fn start_node_impl(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    eth_config: EthConfiguration,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<polkadot_sdk::sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    let mut parachain_config = prepare_node_config(parachain_config);

    let PartialComponents {
        client,
        backend,
        mut task_manager,
        import_queue,
        keystore_container,
        transaction_pool,
        other: (block_import, mut telemetry, telemetry_worker_handle, frontier_backend, _overrides),
        ..
    } = new_partial(&parachain_config, &eth_config)?;
    let transaction_pool=Arc::clone(&*transaction_pool);
    let FrontierPartialComponents {
        filter_pool,
        fee_history_cache,
        fee_history_cache_limit,
    } = new_frontier_partial(&eth_config)?;

    let (relay_chain_interface, collator_key,_,_) = build_relay_chain_interface(
        polkadot_config,
        &parachain_config,
        telemetry_worker_handle,
        &mut task_manager,
        collator_options.clone(),
        hwbench.clone(),
    )
    .await
    .map_err(|e| sc_service::Error::Application(Box::new(e) as Box<_>))?;
let overrides = Arc::new(StorageOverrideHandler::new(client.clone()));
    let validator = parachain_config.role.is_authority();
    let maybe_registry = parachain_config.prometheus_config.as_ref().map(|cfg| &cfg.registry);
    let prometheus_registry = parachain_config.prometheus_registry().cloned();
    let import_queue_service = import_queue.service();
    let net_config: NetCfg<Block, Hash, NetworkWorker<Block, Hash>> =
        sc_network::config::FullNetworkConfiguration::new(
            &parachain_config.network,
            maybe_registry.cloned(),
        );
let transaction_pool=transaction_pool.clone();
    let (network, system_rpc_tx, tx_handler_controller, sync_service) =
        build_network(BuildNetworkParams {
            parachain_config: &parachain_config,
            client: client.clone(),
            transaction_pool: transaction_pool.clone(),
            para_id,
            net_config,
            spawn_handle: task_manager.spawn_handle(),
            relay_chain_interface: relay_chain_interface.clone(),
            import_queue,
            sybil_resistance_level: CollatorSybilResistance::Resistant,
            metrics: <NetworkWorker<Block, Hash> as sc_network::NetworkBackend<Block, Hash>>::register_notification_metrics(
				parachain_config.prometheus_config.as_ref().map(|config| &config.registry),
			),
        })
        .await?;

    if parachain_config.offchain_worker.enabled {
		use futures::FutureExt;

		let offchain_workers =
			sc_offchain::OffchainWorkers::new(sc_offchain::OffchainWorkerOptions {
				runtime_api_provider: client.clone(),
				keystore: Some(keystore_container.keystore()),
				offchain_db: backend.offchain_storage(),
				transaction_pool: Some(OffchainTransactionPoolFactory::new(
					transaction_pool.clone(),
				)),
				network_provider: Arc::new(network.clone()),
				is_validator: parachain_config.role.is_authority(),
				enable_http_requests: false,
				custom_extensions: move |_| vec![],
			})?;
		task_manager.spawn_handle().spawn(
			"offchain-workers-runner",
			"offchain-work",
			offchain_workers.run(client.clone(), task_manager.spawn_handle()).boxed(),
		);
	}

    // Sinks for pubsub notifications.
    // Everytime a new subscription is created, a new mpsc channel is added to the sink pool.
    // The MappingSyncWorker sends through the channel on block import and the subscription emits a notification to the subscriber on receiving a message through this channel.
    // This way we avoid race conditions when using native substrate block import notification stream.
    let pubsub_notification_sinks: fc_mapping_sync::EthereumBlockNotificationSinks<
        fc_mapping_sync::EthereumBlockNotification<Block>,
    > = Default::default();
    let pubsub_notification_sinks = Arc::new(pubsub_notification_sinks);
    let slot_duration = sc_consensus_aura::slot_duration(&*client)?;
    log::info!("Aura slot duration (ms): {}", slot_duration.as_millis());
    let target_gas_price = eth_config.target_gas_price;

    // for ethereum-compatibility rpc.
    parachain_config.rpc.id_provider = Some(Box::new(fc_rpc::EthereumSubIdProvider));

    let _eth_rpc_params = crate::rpc::EthDeps {
        client: client.clone(),
        pool: transaction_pool.clone(),
        graph: transaction_pool.clone(),
        converter: Some(TransactionConverter::<Block>::default()),
        is_authority: parachain_config.role.is_authority(),
        enable_dev_signer: eth_config.enable_dev_signer,
        network: network.clone(),
        sync: sync_service.clone(),
        frontier_backend: frontier_backend.clone(),
        storage_override: overrides.clone(),
        block_data_cache: Arc::new(fc_rpc::EthBlockDataCacheTask::new(
            task_manager.spawn_handle(),
            overrides.clone(),
            eth_config.eth_log_block_cache,
            eth_config.eth_statuses_cache,
            prometheus_registry.clone(),
        )),
        filter_pool: filter_pool.clone(),
        max_past_logs: eth_config.max_past_logs,
        fee_history_cache: fee_history_cache.clone(),
        fee_history_cache_limit,
        execute_gas_limit_multiplier: eth_config.execute_gas_limit_multiplier,
        forced_parent_hashes: None,
        pending_create_inherent_data_providers: move |_: Hash, ()| {
            let slot_duration = slot_duration;
            let target_gas_price = target_gas_price;
            async move {
                let current = sp_timestamp::InherentDataProvider::from_system_time();
                let now_millis: u64 = current.timestamp().as_millis() as u64;
                let slot_ms: u64 = slot_duration.as_millis() as u64;
                // Align timestamp to the start of the CURRENT slot (floor), matching Aura's CurrentSlot
                let aligned_current_slot_ms: u64 = (now_millis / slot_ms) * slot_ms;

                let timestamp = sp_timestamp::InherentDataProvider::new(aligned_current_slot_ms.into());
                let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                    *timestamp,
                    slot_duration,
                );
                let dynamic_fee = fp_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));

                Ok::<(sc_consensus_aura::InherentDataProvider, sp_timestamp::InherentDataProvider, fp_dynamic_fee::InherentDataProvider), sc_service::Error>((slot, timestamp, dynamic_fee))
            }
        },
    };

    let rpc_builder = {
        let client_for_rpc = client.clone();
        let backend_for_rpc = backend.clone();
        let pool_for_rpc = transaction_pool.clone();
        let network_for_rpc = network.clone();
        let sync_for_rpc = sync_service.clone();
        let frontier_backend_for_rpc = frontier_backend.clone();
        let overrides_for_rpc = overrides.clone();
        let fee_history_cache_for_rpc = fee_history_cache.clone();
        let prometheus_registry_for_rpc = prometheus_registry.clone();
        let filter_pool_for_rpc = filter_pool.clone();
        let pubsub_notification_sinks = pubsub_notification_sinks.clone();
        let spawn_handle = task_manager.spawn_handle();

        Box::new(move |subscription_task_executor| {
            let deps = crate::rpc::FullDeps {
                client: client_for_rpc.clone(),
                backend: backend_for_rpc.clone(),
                pool: pool_for_rpc.clone(),
                //deny_unsafe,
                eth: crate::rpc::EthDeps {
                    client: client_for_rpc.clone(),
                    pool: pool_for_rpc.clone(),
                    graph: pool_for_rpc.clone(),
                    converter: Some(TransactionConverter::<Block>::default()),
                    is_authority: parachain_config.role.is_authority(),
                    enable_dev_signer: eth_config.enable_dev_signer,
                    network: network_for_rpc.clone(),
                    sync: sync_for_rpc.clone(),
                    frontier_backend: frontier_backend_for_rpc.clone(),
                    storage_override: overrides_for_rpc.clone(),
                    block_data_cache: Arc::new(fc_rpc::EthBlockDataCacheTask::new(
                        spawn_handle.clone(),
                        overrides_for_rpc.clone(),
                        eth_config.eth_log_block_cache,
                        eth_config.eth_statuses_cache,
                        prometheus_registry_for_rpc.clone(),
                    )),
                    filter_pool: filter_pool_for_rpc.clone(),
                    max_past_logs: eth_config.max_past_logs,
                    fee_history_cache: fee_history_cache_for_rpc.clone(),
                    fee_history_cache_limit,
                    execute_gas_limit_multiplier: eth_config.execute_gas_limit_multiplier,
                    forced_parent_hashes: None,
                    pending_create_inherent_data_providers: move |_, ()| {
                        let slot_duration = slot_duration;
                        let target_gas_price = target_gas_price;
                        async move {
                            let current = sp_timestamp::InherentDataProvider::from_system_time();
                            let now_millis: u64 = current.timestamp().as_millis() as u64;
                            let slot_ms: u64 = slot_duration.as_millis() as u64;
                            let computed_slot: u64 = now_millis / slot_ms;
                            log::info!(
                                "pending-inherent debug: now_ms={}, slot_ms={}, computed_slot={} (no align)",
                                now_millis, slot_ms, computed_slot
                            );
                            // Use current time directly for pending state; proposer sets the real timestamp
                            let timestamp = sp_timestamp::InherentDataProvider::new(now_millis.into());
                            let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                                *timestamp,
                                slot_duration,
                            );
                            let dynamic_fee = fp_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));

                            Ok((slot, timestamp, dynamic_fee))
                        }
                    },
                },
            };

            crate::rpc::create_full(
                deps,
                subscription_task_executor,
                pubsub_notification_sinks.clone(),
            )
            .map_err(Into::into)
        })
    };

    sc_service::spawn_tasks(sc_service::SpawnTasksParams {
        rpc_builder,
		client: client.clone(),
		transaction_pool: transaction_pool.clone(),
		task_manager: &mut task_manager,
		config: parachain_config,
		keystore: keystore_container.keystore(),
		backend: backend.clone(),
		network: network.clone(),
		sync_service: sync_service.clone(),
		system_rpc_tx,
		tx_handler_controller,
		telemetry: telemetry.as_mut(),
    })?;

    spawn_frontier_tasks(
        &task_manager,
        client.clone(),
        backend.clone(),
        frontier_backend.clone(),
        filter_pool.clone(),
        overrides.clone(),
        fee_history_cache.clone(),
        fee_history_cache_limit,
        sync_service.clone(),
        pubsub_notification_sinks.clone(),
    )
    .await;

    if let Some(hwbench) = hwbench {
        polkadot_sdk::sc_sysinfo::print_hwbench(&hwbench);
        // Here you can check whether the hardware meets your chains' requirements. Putting a link
        // in there and swapping out the requirements for your own are probably a good idea. The
        // requirements for a para-chain are dictated by its relay-chain.
        match SUBSTRATE_REFERENCE_HARDWARE.check_hardware(&hwbench, false) {
			Err(err) if validator => {
				log::warn!(
				"⚠️  The hardware does not meet the minimal requirements {} for role 'Authority'.",
				err
			);
			},
			_ => {},
		}

        if let Some(ref mut telemetry) = telemetry {
            let telemetry_handle = telemetry.handle();
            task_manager.spawn_handle().spawn(
                "telemetry_hwbench",
                None,
                polkadot_sdk::sc_sysinfo::initialize_hwbench_telemetry(telemetry_handle, hwbench),
            );
        }
    }

    let announce_block = {
        let sync_service = sync_service.clone();
        Arc::new(move |hash, data| sync_service.announce_block(hash, data))
    };

    let relay_chain_slot_duration = Duration::from_secs(6);

    let overseer_handle = relay_chain_interface
        .overseer_handle()
        .map_err(|e| sc_service::Error::Application(Box::new(e)))?;

    start_relay_chain_tasks(StartRelayChainTasksParams {
        client: client.clone(),
        announce_block: announce_block.clone(),
        para_id,
        relay_chain_interface: relay_chain_interface.clone(),
        task_manager: &mut task_manager,
        da_recovery_profile: if validator {
            DARecoveryProfile::Collator
        } else {
            DARecoveryProfile::FullNode
        },
        import_queue: import_queue_service,
        relay_chain_slot_duration,
        recovery_handle: Box::new(overseer_handle.clone()),
        sync_service: sync_service.clone(),
        prometheus_registry: prometheus_registry.as_ref(),
    })?;

    if validator {
        start_consensus(
            client.clone(),
            block_import,
            prometheus_registry.as_ref(),
            telemetry.as_ref().map(|t| t.handle()),
            &task_manager,
            relay_chain_interface.clone(),
            transaction_pool,
            sync_service.clone(),
            keystore_container.keystore(),
            relay_chain_slot_duration,
            para_id,
            collator_key.expect("Command line arguments do not allow this. qed"),
            overseer_handle,
            announce_block,
            eth_config.target_gas_price,
        )?;
    }

    //start_network.start_network();

    Ok((task_manager, client))
}

/// Build the import queue for the parachain runtime.
fn build_import_queue(
client: Arc<ParachainClient>,
block_import: ParachainBlockImport,
config: &Configuration,
_eth_config: &EthConfiguration,
telemetry: Option<TelemetryHandle>,
task_manager: &TaskManager,
) -> Result<sc_consensus::DefaultImportQueue<Block>, sc_service::Error> {
Ok(
    cumulus_client_consensus_aura::equivocation_import_queue::fully_verifying_import_queue::<
        sp_consensus_aura::sr25519::AuthorityPair,
        _,
        _,
        _,
        _,
    >(
        client,
        block_import,
        move |_, _| async move {
            let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
            Ok(timestamp)
        },
        &task_manager.spawn_essential_handle(),
        config.prometheus_registry(),
        telemetry,
    ),
)
}
fn start_consensus(
    client: Arc<ParachainClient>,
    block_import: ParachainBlockImport,
    prometheus_registry: Option<&Registry>,
    telemetry: Option<TelemetryHandle>,
    task_manager: &TaskManager,
    relay_chain_interface: Arc<dyn RelayChainInterface>,
    transaction_pool: Arc<sc_transaction_pool::TransactionPoolHandle<Block, ParachainClient>>,
    _sync_oracle: Arc<SyncingService<Block>>,
    keystore: KeystorePtr,
    relay_chain_slot_duration: Duration,
    para_id: ParaId,
    collator_key: CollatorPair,
    overseer_handle: OverseerHandle,
    announce_block: Arc<dyn Fn(Hash, Option<Vec<u8>>) + Send + Sync>,
    target_gas_price_u64: u64,
) -> Result<(), sc_service::Error> {
    use cumulus_client_consensus_aura::collators::basic::{
        self as basic_aura, Params as BasicAuraParams,
    };

    let slot_duration = cumulus_client_consensus_aura::slot_duration(&*client)?;

    let proposer_factory = sc_basic_authorship::ProposerFactory::with_proof_recording(
        task_manager.spawn_handle(),
        client.clone(),
        transaction_pool,
        prometheus_registry,
        telemetry.clone(),
    );
    let proposer = Proposer::new(proposer_factory);

    let collator_service = CollatorService::new(
        client.clone(),
        Arc::new(task_manager.spawn_handle()),
        announce_block,
        client.clone(),
    );
    let params = BasicAuraParams {
        create_inherent_data_providers: move |_, ()| {
            let slot_duration = slot_duration;
            let target_gas_price = target_gas_price_u64;
            async move {
                let timestamp = sp_timestamp::InherentDataProvider::from_system_time();
                let slot = sp_consensus_aura::inherents::InherentDataProvider::from_timestamp_and_slot_duration(
                    *timestamp,
                    slot_duration,
                );
                let dynamic_fee = fp_dynamic_fee::InherentDataProvider(U256::from(target_gas_price));
                Ok((slot, timestamp, dynamic_fee))
            }
        },
        block_import,
        para_client: client,
        relay_client: relay_chain_interface,
        keystore,
        collator_key,
        para_id,
        overseer_handle,
        relay_chain_slot_duration,
        proposer,
        collator_service,
        authoring_duration: Duration::from_millis(500),
        collation_request_receiver: None,
    };

    let fut =
        basic_aura::run::<Block, sp_consensus_aura::sr25519::AuthorityPair, _, _, _, _, _, _>(
            params,
        );
    task_manager
        .spawn_essential_handle()
        .spawn("aura", None, fut);

    Ok(())
}

/// Start a parachain node.
pub async fn start_parachain_node(
    parachain_config: Configuration,
    polkadot_config: Configuration,
    eth_config: EthConfiguration,
    collator_options: CollatorOptions,
    para_id: ParaId,
    hwbench: Option<polkadot_sdk::sc_sysinfo::HwBench>,
) -> sc_service::error::Result<(TaskManager, Arc<ParachainClient>)> {
    start_node_impl(
        parachain_config,
        polkadot_config,
        eth_config,
        collator_options,
        para_id,
        hwbench,
    )
    .await
}
