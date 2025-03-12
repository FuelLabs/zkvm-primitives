//! Test fixtures for mainnet blocks

/// Mainnet blocks
#[cfg_attr(
    feature = "enhanced_enums",
    derive(enum_iterator::Sequence, clap::ValueEnum)
)]
#[cfg_attr(feature = "enhanced_enums", clap(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone)]
pub enum MainnetBlocks {
    /// This is an empty block with just a mint tx
    Block_1522295,
    /// This is a block with a mint tx and a contract interaction with multiple assets
    Block_2243673,
    /// This is a block with 513 transfers
    Block_6333890,
}

/// Get prover input for a particular mainnet block
pub fn get_mainnet_block_input(block: MainnetBlocks) -> Vec<u8> {
    match block {
        MainnetBlocks::Block_1522295 => {
            include_bytes!("fixtures/mainnet_blocks/1522295.bin").to_vec()
        }
        MainnetBlocks::Block_2243673 => {
            include_bytes!("fixtures/mainnet_blocks/2243673.bin").to_vec()
        }
        MainnetBlocks::Block_6333890 => {
            include_bytes!("fixtures/mainnet_blocks/6333890.bin").to_vec()
        }
    }
}
