pub mod counter_contract;
pub mod mainnet_blocks;
pub mod opcodes;

#[cfg(feature = "enhanced_enums")]
mod fixtures;

#[cfg(feature = "enhanced_enums")]
pub use fixtures::*;
