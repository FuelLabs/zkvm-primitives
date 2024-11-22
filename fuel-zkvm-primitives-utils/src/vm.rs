use crate::vm::alu::AluInstruction;
use crate::vm::base::AsRepr;
use crate::vm::blob::BlobInstruction;
use crate::vm::control::ControlInstruction;
use crate::vm::memory::MemoryInstruction;

pub mod alu;

pub mod base;
pub mod blob;
pub mod control;
pub mod memory;

// Implemented instructions for the VM
#[derive(Debug)]
pub enum Instruction {
    ALU(AluInstruction),
    CTRL(ControlInstruction),
    MEM(MemoryInstruction),
    BLOB(BlobInstruction),
}

impl AsRepr for Instruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            Instruction::ALU(alu) => alu.repr(),
            Instruction::CTRL(ctrl) => ctrl.repr(),
            Instruction::MEM(mem) => mem.repr(),
            Instruction::BLOB(blob) => blob.repr(),
        }
    }
}
