use fuel_core::{
    chain_config::{ChainConfig, StateConfig, TESTNET_WALLET_SECRETS},
    service::{Config, FuelService},
    state::historical_rocksdb::StateRewindPolicy,
};
use fuel_core_executor::executor::{ExecutionInstance, ExecutionOptions};
use fuel_core_storage::transactional::{AtomicView, HistoricalView};
use fuel_core_types::{
    fuel_crypto::SecretKey,
    fuel_tx::{Bytes32, ConsensusParameters},
};
use fuel_zkvm_primitives_input_provider::{
    relayer_recorder::RelayerRecorder, storage_access_recorder::StorageAccessRecorder,
};
use fuels::{
    accounts::Account,
    prelude::{Provider, WalletUnlocked},
    types::BlockHeight,
};
use fuels_core::types::transaction_builders::{BuildableTransaction, ScriptTransactionBuilder};
use std::{net::SocketAddr, path::Path};

pub use fuel_zkvm_primitives_utils::vm::Instruction;

const CONSENSUS_PARAMETERS: &[u8] = include_bytes!("fixtures/test_consensus_parameters.json");

async fn send_script_transaction(
    instruction: Instruction,
    wallet: &WalletUnlocked,
) -> anyhow::Result<BlockHeight> {
    let script = instruction.repr();

    let mut builder = ScriptTransactionBuilder::default().with_script(script);
    wallet.add_witnesses(&mut builder)?;
    wallet.adjust_for_fee(&mut builder, 0).await?;
    let provider = wallet.provider().expect("No provider");
    let tx = builder.build(provider).await?;

    let tx_id = provider.send_transaction(tx).await?;

    // Sleep to await the transaction inclusion in off chain database.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let inclusion_block_height = provider
        .get_transaction_by_id(&tx_id)
        .await
        .expect("No transaction")
        .expect("No transaction")
        .block_height
        .expect("No block height");

    Ok(inclusion_block_height)
}

pub struct Service {
    pub fuel_node: FuelService,
    pub input: fuel_zkvm_primitives_prover::Input,
}

fn get_config(consensus_parameters: &mut ConsensusParameters, path: &Path) -> Config {
    let state_config = StateConfig::local_testnet();
    let new_base_asset_id = state_config.coins[0].asset_id;

    consensus_parameters.set_base_asset_id(new_base_asset_id);

    let mut chain_config = ChainConfig::local_testnet();
    chain_config.consensus_parameters = consensus_parameters.clone();

    let mut config = Config::local_node_with_configs(chain_config, state_config);
    config.combined_db_config.state_rewind_policy = StateRewindPolicy::RewindFullRange;
    config.combined_db_config.database_path = path.to_path_buf();
    config.utxo_validation = true;
    config.txpool.utxo_validation = true;

    config
}

async fn get_wallet(socket: SocketAddr) -> WalletUnlocked {
    // Get the secret for the genesis wallet
    let secret_key: Bytes32 = TESTNET_WALLET_SECRETS[0]
        .parse()
        .expect("Invalid secret key");
    let secret_key = SecretKey::try_from(secret_key).expect("Invalid secret key");

    let url = format!("http://{}", socket);
    let provider = Provider::connect(url)
        .await
        .expect("Unable to connect to provider");

    WalletUnlocked::new_from_private_key(secret_key, Some(provider))
}

pub async fn start_node_with_transaction_and_produce_prover_input(
    instruction: Instruction,
) -> anyhow::Result<Service> {
    // Suggest to set "RUST_LOG=info;FUEL_TRACE=1" to see the logs
    // If you want to change the block gas limit,
    // please update next values in the `test_test_consensus_parameters.json`:
    // `max_gas_per_tx`, `max_gas_per_predicate` and `block_gas_limit`
    let tmp = tempfile::tempdir().expect("Unable to create temp dir");
    let mut consensus_parameters =
        serde_json::from_slice::<ConsensusParameters>(CONSENSUS_PARAMETERS).expect("Invalid JSON");

    let fuel_node =
        FuelService::new_node(get_config(&mut consensus_parameters, tmp.path())).await?;

    let wallet = get_wallet(fuel_node.bound_address).await;
    let tx_inclusion_block_height = send_script_transaction(instruction, &wallet).await?;

    let on_chain_database = fuel_node.shared.database.on_chain();
    let block_height_before_tx = tx_inclusion_block_height.pred().expect("Impossible");
    let on_chain_storage_at_height = on_chain_database.view_at(&block_height_before_tx)?;

    // We don't need to specify the height for the relayer.
    // Relayer stores events for all height from DA.
    let latest_relayer = fuel_node.shared.database.relayer().latest_view()?;

    let storage = StorageAccessRecorder::new(on_chain_storage_at_height);
    let relayer = RelayerRecorder::new(latest_relayer);

    let validator = ExecutionInstance::new(
        relayer.clone(),
        storage.clone(),
        ExecutionOptions {
            extra_tx_checks: true,
            backtrace: false,
        },
    );

    let block = on_chain_database
        .latest_view()?
        .get_full_block(&tx_inclusion_block_height)?
        .expect("Block with transaction is not available");
    let _ = validator.validate_without_commit(&block)?;

    let input = fuel_zkvm_primitives_prover::Input {
        block,
        storage: storage.into_changes(),
        relayer: relayer.into_prover_relayer(),
    };

    Ok(Service { fuel_node, input })
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use fuel_zkvm_primitives_utils::logs::init_logging;

    async fn basic_alu_test(instruction: Instruction) {
        init_logging();

        let service = start_node_with_transaction_and_produce_prover_input(instruction)
            .await
            .unwrap();
        let serialized_input = bincode::serialize(&service.input).unwrap();
        let proof = fuel_zkvm_primitives_prover::prove(&serialized_input).unwrap();
        let block_id: [u8; 32] = service.input.block.header().id().into();
        assert_eq!(block_id, proof.block_id.to_be_bytes())
    }

    #[tokio::test]
    async fn prover_can_verify__alu__add() {
        basic_alu_test(Instruction::ADDI).await;
    }

    #[tokio::test]
    async fn prover_can_verify__alu__mul() {
        basic_alu_test(Instruction::MULI).await;
    }
}
