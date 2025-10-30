use cumulus_primitives_core::ParaId;
use fintradex_runtime::{AccountId, AuraId, Signature, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public, H160};
use sp_runtime::traits::{IdentifyAccount, Verify};
use std::{collections::BTreeMap, str::FromStr};
use polkadot_sdk::{staging_xcm as xcm, *};
use fintradex_runtime::{WASM_BINARY,BlockNumber};
const PARA_ID: u32 = 5023;
const PROTOCOL_ID: &str = "fint";
/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
    /// The EVM since block number.
    pub evm_since: BlockNumber,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
    get_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(keys: AuraId) -> fintradex_runtime::SessionKeys {
    fintradex_runtime::SessionKeys { aura: keys }
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    //let wasm_binary = WASM_BINARY.ok_or("Development wasm binary not available".to_string());
let wasm_binary = WASM_BINARY.expect("WASM not available");
	let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "Fint".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

	let genesis_json = testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        // Give Alice root privileges
        Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
        1000.into(),
    );

	let chain_spec = ChainSpec::builder(
		wasm_binary,
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
			evm_since: 1,
		},
	)
	.with_name("Fintradex Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Development)
	.with_boot_nodes(vec![])
	.with_properties(properties)
	.with_protocol_id(PROTOCOL_ID)
	.with_genesis_config_patch(genesis_json)
	.build();

	chain_spec
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let wasm_binary = WASM_BINARY.expect("WASM not available");
	let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "Fint".into());
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("ss58Format".into(), 42.into());

	let genesis_json = testnet_genesis(
        // initial collators.
        vec![
            (
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                get_collator_keys_from_seed("Alice"),
            ),
            (
                get_account_id_from_seed::<sr25519::Public>("Bob"),
                get_collator_keys_from_seed("Bob"),
            ),
        ],
        vec![
            get_account_id_from_seed::<sr25519::Public>("Alice"),
            get_account_id_from_seed::<sr25519::Public>("Bob"),
            get_account_id_from_seed::<sr25519::Public>("Charlie"),
            get_account_id_from_seed::<sr25519::Public>("Dave"),
            get_account_id_from_seed::<sr25519::Public>("Eve"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie"),
            get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
            get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
            get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
            get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
            get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
            get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        ],
        // Give Alice root privileges
        Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
        1000.into(),
    );

	let chain_spec = ChainSpec::builder(
		wasm_binary,
		Extensions {
			relay_chain: "rococo-local".into(),
			para_id: PARA_ID,
			evm_since: 1,
		},
	)
	.with_name("Fintradex Local Testnet")
	.with_id("local_testnet")
	.with_chain_type(ChainType::Local)
	.with_boot_nodes(vec![])
	.with_properties(properties)
	.with_protocol_id(PROTOCOL_ID)
	.with_genesis_config_patch(genesis_json)
	.build();

	chain_spec
}

fn testnet_genesis(
    invulnerables: Vec<(AccountId, AuraId)>,
    endowed_accounts: Vec<AccountId>,
    root_key: Option<AccountId>,
    id: ParaId,
) -> serde_json::Value  {
    // Explicit AccountId conversions to avoid inference inside json! macro
    let alice_acc: AccountId = get_account_id_from_seed::<sr25519::Public>("Alice");
    let bob_acc: AccountId = get_account_id_from_seed::<sr25519::Public>("Bob");
    let asset1: String = "asset-1".to_string();
    let asset2: String = "asset-2".to_string();
    let alt_1: String = "ALT1".to_string();
    let alt_2: String = "ALT2".to_string();

    // Build EVM genesis accounts outside the json! macro to avoid token-tree errors
    let evm_accounts = {
        let mut map: BTreeMap<sp_core::H160, serde_json::Value> = BTreeMap::new();
        map.insert(
            H160::from_str("d43593c715fdd31c61141abd04a99fd6822c8558").expect("valid H160"),
            serde_json::json!({
                "balance": "0xffffffffffffffffffffffffffffffff",
                "code": "0x",
                "nonce": 0,
                "storage": {}
            }),
        );
        map.insert(
            H160::from_str("6be02d1d3665660d22ff9624b7be0551ee1ac91b").expect("valid H160"),
            serde_json::json!({
                "balance": "0xffffffffffffffffffffffffffffffff",
                "code": "0x",
                "nonce": 0,
                "storage": {}
            }),
        );
        map.insert(
            H160::from_str("1000000000000000000000000000000000000001").expect("valid H160"),
            serde_json::json!({
                "nonce": "0x1",
                "balance": "0x0de0b6b3a7640000000000", // 1e24 approx in hex
                "storage": {},
                "code": "0x00",
            }),
        );
        map.insert(
            H160::from_str("c0f0f4ab324c46e55d02d0033343b4be8a55532d").expect("valid H160"),
            serde_json::json!({
                "balance": "0x0ef0000000000000000000000000000",
                "code": "0x",
                "nonce": 0,
                "storage": {}
            }),
        );
        serde_json::to_value(map).expect("serialize evm accounts")
    };

    // Convert all complex types to serde_json::Value to help type inference
    let root_key_value = serde_json::to_value(root_key).expect("serialize root_key");
    let id_value = serde_json::to_value(id).expect("serialize para_id");
    let safe_xcm_version_value = serde_json::to_value(SAFE_XCM_VERSION).expect("serialize safe_xcm_version");
    
    // Convert collections to serde_json::Value
    let assets_list = serde_json::to_value(vec![
        (1, alice_acc.clone(), true, 10_000_000_0000u128),
        (2, bob_acc.clone(), true, 10_000_000_0000u128),
    ]).expect("serialize assets");
    let metadata_list = serde_json::to_value(vec![
        (1, asset1.clone(), alt_1.clone(), 10),
        (2, asset2.clone(), alt_2.clone(), 10),
    ]).expect("serialize metadata");
    let accounts_list = serde_json::to_value(vec![
        (1, alice_acc.clone(), 50_000_000_0000u128),
        (2, bob_acc.clone(), 50_000_000_0000u128),
    ]).expect("serialize accounts");
    let balances_list = serde_json::to_value(
        endowed_accounts.iter().cloned().map(|k: AccountId| (k, 1u128 << 60)).collect::<Vec<(AccountId, u128)>>()
    ).expect("serialize balances");
    let invulnerables_list = serde_json::to_value(
        invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<AccountId>>()
    ).expect("serialize invulnerables");
    let session_keys_list = serde_json::to_value(
        invulnerables.iter().map(|(acc, aura)| {
            (acc.clone(), acc.clone(), template_session_keys(aura.clone()))
        }).collect::<Vec<(AccountId, AccountId, fintradex_runtime::SessionKeys)>>()
    ).expect("serialize session keys");

    let genesis_json: serde_json::Value = serde_json::json!({ 
        "system": {},
        // Configure additional assets here
        // For example, this configures asset "ALT1" & "ALT2" with owners, alice and bob, respectively
        "assets": {
            "assets": assets_list,
            "metadata": metadata_list,
            "next_asset_id": 3,
            "accounts": accounts_list,
        },
        "balances": {
            "balances": balances_list,
            "dev_accounts": balances_list,
        },
        "parachain_info": {
            "parachain_id": id_value
        },
        "collator_selection": {
            "invulnerables": invulnerables_list,
            "candidacy_bond": EXISTENTIAL_DEPOSIT * 16
        },
        "session": {
            "keys": session_keys_list,
            "non_authority_keys": serde_json::json!([]),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        "aura": {},
        "aura_ext": {},
        "parachain_system": {},
        "polkadot_xcm": {
            "safe_xcm_version": safe_xcm_version_value
        },
        "sudo": {
            "key": root_key_value,
        },
        "transaction_payment": {},
        // EVM compatibility
        "evm_chain_id": {
            "chain_id": 1000
        },
        "evm": {
            "accounts": evm_accounts
        },
        "ethereum": {},
        //dynamic_fee: Default::default(),
        "base_fee": {}
    });
    
    genesis_json
}