use crate::vm::AsRepr;
use fuel_core_types::fuel_asm::{op, RegId};

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum ControlInstruction {
    JMP,
    JI,
    JNE,
    JNEI,
    JNZI,
    JMPB,
    JMPF,
    JNZB,
    JNZF,
    JNEB,
    JNEF,
    // ignoring this because we can't infinite loop this :_)
    // RET
}

impl AsRepr for ControlInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            ControlInstruction::JMP => vec![op::movi(0x10, 0), op::jmp(0x10)],
            ControlInstruction::JI => vec![op::ji(0), op::jmpb(RegId::ZERO, 0)],
            ControlInstruction::JNE => vec![
                op::movi(0x10, 0),
                op::jne(RegId::ZERO, RegId::ONE, 0x10),
                op::jmpb(RegId::ZERO, 0),
            ],
            ControlInstruction::JNEI => vec![
                op::jnei(RegId::ZERO, RegId::ONE, 0),
                op::jmpb(RegId::ZERO, 0),
            ],
            ControlInstruction::JNZI => vec![op::jnzi(RegId::ONE, 0), op::jmpb(RegId::ZERO, 0)],
            ControlInstruction::JMPB => vec![op::noop(), op::jmpb(RegId::ZERO, 0)],
            ControlInstruction::JMPF => vec![op::jmpf(RegId::ZERO, 0), op::jmpb(RegId::ZERO, 0)],
            ControlInstruction::JNZB => vec![
                op::movi(0x10, 1),
                op::noop(),
                op::jnzb(0x10, RegId::ZERO, 0),
            ],
            ControlInstruction::JNZF => vec![
                op::movi(0x10, 1),
                op::noop(),
                op::jnzf(0x10, RegId::ZERO, 1),
                op::ret(RegId::ZERO),
                op::jmpb(RegId::ZERO, 1),
            ],
            ControlInstruction::JNEB => vec![
                op::movi(0x10, 1),
                op::movi(0x11, 0),
                op::noop(),
                op::jneb(0x10, 0x11, RegId::ZERO, 0),
            ],
            ControlInstruction::JNEF => vec![
                op::movi(0x10, 1),
                op::movi(0x11, 1),
                op::noop(),
                op::jnef(0x10, 0x11, RegId::ZERO, 1),
                op::jmpb(RegId::ZERO, 0),
                op::noop(),
            ],
        }
        .into_iter()
        .collect()
    }
}
