#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::utils::{generate_input_at_block_height, Service};
    use fuel_core::chain_config::{ChainConfig, SnapshotMetadata, StateConfig};
    use fuel_core::service::{Config, FuelService};
    use fuel_core::state::historical_rocksdb::StateRewindPolicy;
    use fuel_core_types::fuel_tx::ConsensusParameters;
    use std::path::Path;

    const CONSENSUS_PARAMETERS: &[u8] =
        include_bytes!("fixtures/testnet_block/block_1360410_consensus_parameters.json");

    fn get_config(metadata_path: &Path, db_path: &Path) -> Config {
        let mut chain_config =
            ChainConfig::from_snapshot_metadata(&SnapshotMetadata::read(&metadata_path).unwrap())
                .unwrap();
        let state_config =
            StateConfig::from_snapshot_metadata(SnapshotMetadata::read(&metadata_path).unwrap())
                .unwrap();

        let mut consensus_parameters =
            serde_json::from_slice::<ConsensusParameters>(CONSENSUS_PARAMETERS)
                .expect("Invalid JSON");

        chain_config.consensus_parameters = consensus_parameters.clone();

        let mut config = Config::local_node_with_configs(chain_config, state_config);
        config.combined_db_config.state_rewind_policy = StateRewindPolicy::RewindFullRange;
        config.combined_db_config.database_path = db_path.to_path_buf();
        config.utxo_validation = true;
        config.txpool.utxo_validation = true;

        config
    }

    async fn start_node_and_produce_prover_input() -> anyhow::Result<Service> {
        let metadata_path = Path::new("fixtures/testnet_block");
        // just a one-time thing
        let db_path = Path::new("fixtures/testnet_block/db");

        let fuel_node = FuelService::new_node(get_config(&metadata_path, db_path.path())).await?;

        let service = generate_input_at_block_height(fuel_node, 1360410.into()).await?;

        // generate serialized input
        let serialized_input = bincode::serialize(&service.input)?;
        std::fs::write("fixtures/testnet_block/1360410.bin", &serialized_input)?;

        Ok(service)
    }

    #[tokio::test]
    async fn test_testnet_block_1360410() {
        let service = start_node_and_produce_prover_input().await.unwrap();
        // let serialized_input = bincode::serialize(&service.input).unwrap();
        // let proof = fuel_zkvm_primitives_prover::prove(&serialized_input).unwrap();
        // let block_id: [u8; 32] = service.input.block.header().id().into();
        // assert_eq!(block_id, proof.block_id.to_be_bytes())
    }
}
