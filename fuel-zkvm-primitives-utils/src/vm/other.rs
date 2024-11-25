use crate::vm::AsRepr;
use fuel_core_types::fuel_asm::{op, GMArgs, GTFArgs, RegId};

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, PartialEq, Clone)]
pub enum OtherInstruction {
    FLAG,
    GTF,
    GM,
    // ECAL. skipped
}

impl AsRepr for OtherInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            OtherInstruction::FLAG => vec![op::flag(0x10), op::jmpb(RegId::ZERO, 0)],
            OtherInstruction::GTF => vec![
                op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
                op::jmpb(RegId::ZERO, 0),
            ],
            OtherInstruction::GM => vec![
                op::gm(0x10, GMArgs::GetChainId.into()),
                op::jmpb(RegId::ZERO, 0),
            ],
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        match &self {
            OtherInstruction::GTF => Some(vec![0x10, 0x11, 0x12, 0x13]),
            _ => None,
        }
    }
}
