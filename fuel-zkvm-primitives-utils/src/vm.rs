use crate::vm::alu::AluInstruction;
use crate::vm::base::AsRepr;
use crate::vm::blob::BlobInstruction;
use crate::vm::control::ControlInstruction;
use crate::vm::crypto::CryptoInstruction;
use crate::vm::memory::MemoryInstruction;
use crate::vm::other::OtherInstruction;

pub mod alu;
pub mod base;
pub mod blob;
pub mod control;
pub mod crypto;
pub mod memory;
pub mod other;

// Implemented instructions for the VM
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
