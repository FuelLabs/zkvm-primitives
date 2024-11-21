#![allow(unused)]

pub enum MainnetBlocks {
    // This is an empty block with just a mint tx
    Block1522295,
}

pub fn get_mainnet_block_input(block: MainnetBlocks) -> Vec<u8> {
    match block {
        MainnetBlocks::Block1522295 => {
            include_bytes!("fixtures/mainnet_blocks/1522295.bin").to_vec()
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mainnet_blocks::get_mainnet_block_input;
    use fuel_zkvm_primitives_prover::{prove, Input};

    #[tokio::test]
    async fn test_mainnet_block_1522295() {
        let serialized_input = get_mainnet_block_input(MainnetBlocks::Block1522295);

        let proof = prove(&serialized_input).unwrap();
        let deserialized_input: Input = bincode::deserialize(&serialized_input).unwrap();
        let block_id: [u8; 32] = deserialized_input.block.header().id().into();

        assert_eq!(block_id, proof.block_id.to_be_bytes())
    }
}
