use crate::mainnet_blocks::{get_mainnet_block_input, MainnetBlocks};
use crate::opcodes::get_opcode_input;
use clap::builder::PossibleValue;
use clap::ValueEnum;
use fuel_zkvm_primitives_utils::vm::Instruction;
use std::sync::OnceLock;

static FIXTURE_VARIANTS: OnceLock<Vec<Fixture>> = OnceLock::new();

pub fn all_fixtures() -> &'static Vec<Fixture> {
    FIXTURE_VARIANTS.get_or_init(|| enum_iterator::all::<Fixture>().collect())
}

#[derive(Debug, Clone, enum_iterator::Sequence)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Fixture {
    MainnetBlock(MainnetBlocks),
    Opcode(Instruction),
}

impl Fixture {
    pub fn get_input_for_fixture(fixture: &Fixture) -> Vec<u8> {
        match fixture {
            Fixture::MainnetBlock(block) => get_mainnet_block_input(*block),
            Fixture::Opcode(instruction) => get_opcode_input(instruction.clone()),
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
        }
    }
}
