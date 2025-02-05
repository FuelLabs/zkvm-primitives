use fuel_core::chain_config::{ChainConfig, StateConfig, TESTNET_WALLET_SECRETS};
use fuel_core::combined_database::CombinedDatabase;
use fuel_core::database::database_description::on_chain::OnChain;
use fuel_core::database::{Database, RegularStage};
use fuel_core::service::{Config, FuelService};
use fuel_core::state::data_source::DataSource;
use fuel_core::state::historical_rocksdb::description::Historical;
use fuel_core::state::historical_rocksdb::{HistoricalRocksDB, StateRewindPolicy};
use fuel_core::state::rocks_db::RocksDb;
use fuel_core_executor::executor::{ExecutionInstance, ExecutionOptions};
use fuel_core_storage::transactional::{AtomicView, HistoricalView};
use fuel_core_types::fuel_crypto::SecretKey;
use fuel_core_types::fuel_tx::{Bytes32, ConsensusParameters};
use fuel_core_types::fuel_types::BlockHeight;
use fuel_zkvm_primitives_input_provider::relayer_recorder::RelayerRecorder;
use fuel_zkvm_primitives_input_provider::storage_access_recorder::StorageAccessRecorder;
use fuel_zkvm_primitives_prover::games::block_execution_game;
use fuels::prelude::{Provider, WalletUnlocked};
use std::net::SocketAddr;
use std::path::Path;
use std::sync::Arc;

const CONSENSUS_PARAMETERS: &[u8] =
    include_bytes!("../src/fixtures/test_consensus_parameters.json");

pub struct Service {
    #[allow(dead_code)]
    pub fuel_node: FuelService,
    pub input: block_execution_game::Input,
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

pub fn get_temp_db() -> Database<OnChain> {
    let db = RocksDb::<Historical<OnChain>>::default_open_temp().unwrap();
    let historical_db = HistoricalRocksDB::new(db, StateRewindPolicy::RewindFullRange).unwrap();
    let data = Arc::new(historical_db);
    Database::from_storage(DataSource::new(data, RegularStage::default()))
}

pub async fn start_node_with_db(
    db: Database<OnChain>,
    consensus_parameters: Option<ConsensusParameters>,
) -> (FuelService, WalletUnlocked) {
    let mut consensus_parameters = consensus_parameters.unwrap_or_else(|| {
        serde_json::from_slice::<ConsensusParameters>(CONSENSUS_PARAMETERS).expect("Invalid JSON")
    });

    let config = get_config(&mut consensus_parameters, Path::new("/tmp"));

    let fuel_node = FuelService::from_combined_database(
        CombinedDatabase::new(
            db,
            Default::default(),
            Default::default(),
            Default::default(),
        ),
        config,
    )
    .await
    .unwrap();

    let wallet = get_wallet(fuel_node.bound_address).await;

    (fuel_node, wallet)
}

pub async fn start_node(
    consensus_parameters: Option<ConsensusParameters>,
) -> (FuelService, WalletUnlocked) {
    // Suggest to set "RUST_LOG=info;FUEL_TRACE=1" to see the logs
    // If you want to change the block gas limit,
    // please update next values in the `test_test_consensus_parameters.json`:
    // `max_gas_per_tx`, `max_gas_per_predicate` and `block_gas_limit`
    let tmp = tempfile::tempdir().expect("Unable to create temp dir");
    let mut consensus_parameters = consensus_parameters.unwrap_or_else(|| {
        serde_json::from_slice::<ConsensusParameters>(CONSENSUS_PARAMETERS).expect("Invalid JSON")
    });

    let fuel_node = FuelService::new_node(get_config(&mut consensus_parameters, tmp.path()))
        .await
        .unwrap();
    let wallet = get_wallet(fuel_node.bound_address).await;

    (fuel_node, wallet)
}

pub async fn generate_input_at_block_height(
    fuel_node: FuelService,
    height: BlockHeight,
) -> anyhow::Result<Service> {
    let on_chain_database = fuel_node.shared.database.on_chain();
    let block_height_before_tx = height.pred().expect("Impossible");
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
        .get_full_block(&height)?
        .expect("Block with transaction is not available");
    let _ = validator.validate_without_commit(&block)?;

    let input = block_execution_game::Input {
        block,
        storage: storage.into_changes(),
        relayer: relayer.into_prover_relayer(),
    };

    Ok(Service { fuel_node, input })
}
