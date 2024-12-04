#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use fuel_zkvm_primitives_prover::{prove, Input};

    #[tokio::test]
    async fn test_counter_contract__increment() {
        let serialized_input = include_bytes!("fixtures/counter_contract/input.bin").to_vec();
        let proof = prove(&serialized_input).unwrap();
        let deserialized_input: Input = bincode::deserialize(&serialized_input).unwrap();
        let block_id: [u8; 32] = deserialized_input.block.header().id().into();

        assert_eq!(block_id, proof.block_id.to_be_bytes())
    }
}
