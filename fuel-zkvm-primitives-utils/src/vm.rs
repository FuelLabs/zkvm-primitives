use crate::vm::alu::AluInstruction;
use crate::vm::base::AsRepr;
use crate::vm::blob::BlobInstruction;
use crate::vm::control::ControlInstruction;
use crate::vm::crypto::CryptoInstruction;
use crate::vm::memory::MemoryInstruction;
use crate::vm::other::OtherInstruction;
use enum_iterator::all;
use std::sync::OnceLock;

pub mod alu;
pub mod base;
pub mod blob;
pub mod control;
pub mod crypto;
pub mod memory;
pub mod other;

static INSTRUCTION_VARIANTS: OnceLock<Vec<Instruction>> = OnceLock::new();

pub fn all_instructions() -> &'static Vec<Instruction> {
    INSTRUCTION_VARIANTS.get_or_init(|| all::<Instruction>().collect())
}

// Implemented instructions for the VM
#[cfg_attr(feature = "enhanced_enums", derive(enum_iterator::Sequence))]
#[derive(Debug, Clone)]
pub enum Instruction {
    ALU(AluInstruction),
    CTRL(ControlInstruction),
    MEM(MemoryInstruction),
    BLOB(BlobInstruction),
    CRYPTO(CryptoInstruction),
    OTHER(OtherInstruction),
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
        }
    }
}
