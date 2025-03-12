use crate::vm::alu::AluInstruction;
use crate::vm::base::AsRepr;
use crate::vm::blob::BlobInstruction;
use crate::vm::contract::ContractInstruction;
use crate::vm::control::ControlInstruction;
use crate::vm::crypto::CryptoInstruction;
use crate::vm::memory::MemoryInstruction;
use crate::vm::other::OtherInstruction;
use crate::vm::zk::ZkInstruction;
use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;

pub mod alu;
pub mod base;
pub mod blob;
pub mod contract;
pub mod control;
pub mod crypto;
pub mod memory;
pub mod other;
pub mod utils;
pub mod zk;

#[cfg(feature = "enhanced_enums")]
static INSTRUCTION_VARIANTS: std::sync::OnceLock<Vec<Instruction>> = std::sync::OnceLock::new();

#[cfg(feature = "enhanced_enums")]
pub fn all_instructions() -> &'static Vec<Instruction> {
    INSTRUCTION_VARIANTS.get_or_init(|| enum_iterator::all::<Instruction>().collect())
}

// Implemented instructions for the VM
#[cfg_attr(feature = "enhanced_enums", derive(enum_iterator::Sequence))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum Instruction {
    ALU(AluInstruction),
    CTRL(ControlInstruction),
    MEM(MemoryInstruction),
    BLOB(BlobInstruction),
    CRYPTO(CryptoInstruction),
    OTHER(OtherInstruction),
    CONTRACT(ContractInstruction),
    ZK(ZkInstruction),
}

impl AsRepr for Instruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            Instruction::ALU(alu) => alu.repr(),
            Instruction::CTRL(ctrl) => ctrl.repr(),
            Instruction::MEM(mem) => mem.repr(),
            Instruction::BLOB(blob) => blob.repr(),
            Instruction::CRYPTO(crypto) => crypto.repr(),
            Instruction::OTHER(other) => other.repr(),
            Instruction::CONTRACT(contract) => contract.repr(),
            Instruction::ZK(zk) => zk.repr(),
        }
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        match &self {
            Instruction::ALU(alu) => alu.script_data(),
            Instruction::CTRL(ctrl) => ctrl.script_data(),
            Instruction::MEM(mem) => mem.script_data(),
            Instruction::BLOB(blob) => blob.script_data(),
            Instruction::CRYPTO(crypto) => crypto.script_data(),
            Instruction::OTHER(other) => other.script_data(),
            Instruction::CONTRACT(contract) => contract.script_data(),
            Instruction::ZK(zk) => zk.script_data(),
        }
    }

    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        match &self {
            Instruction::CONTRACT(contract) => contract.additional_inputs(),
            _ => None,
        }
    }

    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        match &self {
            Instruction::CONTRACT(contract) => contract.additional_outputs(),
            _ => None,
        }
    }
}

#[cfg(feature = "enhanced_enums")]
impl clap::ValueEnum for Instruction {
    fn value_variants<'a>() -> &'a [Self] {
        all_instructions().as_slice()
    }
    fn to_possible_value<'a>(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Instruction::ALU(i) => i.to_possible_value(),
            Instruction::CTRL(i) => i.to_possible_value(),
            Instruction::MEM(i) => i.to_possible_value(),
            Instruction::BLOB(i) => i.to_possible_value(),
            Instruction::CRYPTO(i) => i.to_possible_value(),
            Instruction::OTHER(i) => i.to_possible_value(),
            Instruction::CONTRACT(i) => i.to_possible_value(),
            Instruction::ZK(i) => i.to_possible_value(),
        }
    }
}
