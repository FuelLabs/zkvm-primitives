use crate::vm::alu::AluInstruction;
use crate::vm::base::AsRepr;

pub mod alu;
pub mod base;

// Implemented instructions for the VM
#[derive(Debug)]
pub enum Instruction {
    ALU(AluInstruction),
}

impl AsRepr for Instruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            Instruction::ALU(alu) => alu.repr(),
        }
    }
}
