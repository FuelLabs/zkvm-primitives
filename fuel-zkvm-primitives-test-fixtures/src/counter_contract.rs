#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::utils::{generate_input_at_block_height, start_node, Service};
    use fuel_core_types::fuel_tx::ConsensusParameters;
    use fuels::prelude::*;

    const COUNTER_CONTRACT_CONSENSUS_PARAMETERS: &[u8] =
        include_bytes!("fixtures/counter_contract/test_consensus_parameters.json");

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

    #[tokio::test]
    async fn test_counter_contract__increment() {
        let service = start_node_with_transaction_and_produce_prover_input()
            .await
            .unwrap();

        let serialized_input = bincode::serialize(&service.input).unwrap();
        let proof = fuel_zkvm_primitives_prover::prove(&serialized_input).unwrap();
        let block_id: [u8; 32] = service.input.block.header().id().into();
        assert_eq!(proof.block_id.to_be_bytes(), block_id);
    }
}
