//! All the fixtures for the prover

use crate::block_execution_fixtures::counter_contract::get_counter_contract_input;
use crate::block_execution_fixtures::mainnet_blocks::{get_mainnet_block_input, MainnetBlocks};
use crate::block_execution_fixtures::opcodes::get_opcode_input;
use clap::builder::PossibleValue;
use fuel_zkvm_primitives_utils::vm::Instruction;
use std::sync::OnceLock;

static FIXTURE_VARIANTS: OnceLock<Vec<Fixture>> = OnceLock::new();

/// Get all fixtures
pub fn all_fixtures() -> &'static Vec<Fixture> {
    FIXTURE_VARIANTS.get_or_init(|| enum_iterator::all::<Fixture>().collect())
}

/// Fixtures for the prover
#[derive(Debug, Clone, enum_iterator::Sequence)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fixture {
    /// Mainnet block fixture
    MainnetBlock(MainnetBlocks),
    /// Opcode fixture
    Opcode(Instruction),
    /// Counter contract fixture
    CounterContract,
}

impl Fixture {
    /// Get the prover input for the fixture
    pub fn get_input_for_fixture(&self) -> Vec<u8> {
        match self {
            Fixture::MainnetBlock(block) => get_mainnet_block_input(*block),
            Fixture::Opcode(instruction) => get_opcode_input(instruction.clone()),
            Fixture::CounterContract => get_counter_contract_input(),
        }
    }
}

impl clap::ValueEnum for Fixture {
    fn value_variants<'a>() -> &'a [Self] {
        all_fixtures().as_slice()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Fixture::MainnetBlock(block) => block.to_possible_value(),
            Fixture::Opcode(instruction) => instruction.to_possible_value(),
            Fixture::CounterContract => Some(PossibleValue::new("counter-contract")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fuel_zkvm_primitives_prover::games::block_execution_game::{prove, Input};
    use rayon::prelude::*;

    #[test]
    fn test_all_fixtures() {
        let fixtures = all_fixtures();

        fixtures.par_iter().for_each(|fixture| {
            if let Err(err) = (|| -> Result<(), String> {
                let prover_input = fixture.get_input_for_fixture();

                if prover_input.is_empty() {
                    return Err(format!("Fixture '{:?}' has empty prover input", fixture));
                }

                let deserialized_input: Input =
                    bincode::deserialize(&prover_input).map_err(|e| {
                        format!("Failed to deserialize fixture '{:?}': {:?}", fixture, e)
                    })?;

                let proof = prove(&prover_input).map_err(|e| {
                    format!(
                        "Failed to generate proof for fixture '{:?}': {:?}",
                        fixture, e
                    )
                })?;

                let block_id: [u8; 32] = deserialized_input.block.header().id().into();

                if block_id != proof.block_id.to_be_bytes() {
                    return Err(format!("Fixture '{:?}' failed: block ID mismatch", fixture));
                }

                Ok(())
            })() {
                panic!("{}", err);
            }
        });
    }
}
