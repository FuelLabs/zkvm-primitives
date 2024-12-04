use crate::utils::{generate_input_at_block_height, start_node, Service};
use forc::cli::shared::{Build, BuildOutput, Pkg};
use forc::cli::BuildCommand;
use forc::ops::forc_build;
use fuel_core_types::fuel_tx::ConsensusParameters;
use fuels::prelude::*;
use std::path::Path;

const COUNTER_CONTRACT_CONSENSUS_PARAMETERS: &[u8] =
    include_bytes!("../src/fixtures/counter_contract/test_consensus_parameters.json");

abigen!(Contract(name = "Counter", abi = "fuel-zkvm-primitives-test-fixtures/src/fixtures/counter_contract/out/counter_contract-abi.json"));

async fn deploy(wallet: WalletUnlocked) -> (Counter<WalletUnlocked>, ContractId) {
    let id = Contract::load_from(
        "src/fixtures/counter_contract/out/counter_contract.bin",
        LoadConfiguration::default().with_storage_configuration(
            StorageConfiguration::default()
                .add_slot_overrides_from_file(
                    "src/fixtures/counter_contract/out/counter_contract-storage_slots.json",
                )
                .unwrap(),
        ),
    )
    .unwrap()
    .deploy(&wallet, TxPolicies::default())
    .await
    .unwrap();

    let instance = Counter::new(id.clone(), wallet);

    (instance, id.into())
}

async fn start_node_with_transaction_and_produce_prover_input() -> anyhow::Result<Service> {
    let (fuel_node, wallet) = start_node(Some(
        serde_json::from_slice::<ConsensusParameters>(COUNTER_CONTRACT_CONSENSUS_PARAMETERS)
            .expect("Invalid JSON"),
    ))
    .await;

    let (contract, _) = deploy(wallet.clone()).await;

    let result = contract.methods().increment().call().await?;

    let provider = wallet.provider().expect("No provider");
    let tx_inclusion_block_height = provider
        .get_transaction_by_id(&result.tx_id.expect("No tx id"))
        .await
        .expect("No transaction")
        .expect("No transaction")
        .block_height
        .expect("No block height");

    let service = generate_input_at_block_height(fuel_node, tx_inclusion_block_height).await?;
    Ok(service)
}

pub async fn generate_fixture() -> anyhow::Result<()> {
    let base_path = Path::new("src/fixtures/counter_contract");
    let out_dir = base_path.join("out");
    let out_bin = out_dir.join("counter_contract.bin");

    // create out_dir if it doesn't exist
    std::fs::create_dir_all(out_dir).expect("Failed to create out directory");

    let build_command = BuildCommand {
        build: Build {
            pkg: Pkg {
                path: Some(base_path.display().to_string()),
                offline: false,
                terse: false,
                output_directory: None,
                locked: false,
                ipfs_node: None,
            },
            print: Default::default(),
            minify: Default::default(),
            output: BuildOutput {
                bin_file: Some(out_bin.display().to_string()),
                debug_file: None,
            },
            profile: Default::default(),
            build_target: Default::default(),
        },
        tests: false,
        experimental: Default::default(),
    };

    forc_build::build(build_command).expect("Failed to build contract");

    let service = start_node_with_transaction_and_produce_prover_input().await?;

    let serialized_input = bincode::serialize(&service.input)?;
    std::fs::write(base_path.join("input.bin"), serialized_input)?;

    Ok(())
}
