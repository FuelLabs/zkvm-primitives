use crate::vm::base::AsRepr;
use fuel_core_types::fuel_asm::{op, RegId};
use fuel_core_types::fuel_types::Bytes32;
use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum ContractInstruction {
    // BAL,
    BHEI,
    BHSH,
    // BURN,
    // CALL,
    CB,
    // CCP,
    // CROO,
    // CSIZ,
    // LDC,
    LOG,
    // LOGD,
    // MINT,
    // RETD,
    // RVRT, Skipped.
    // SMO,
    // SCWQ,
    // SRW,
    // SRWQ,
    // SWW,
    // SWWQ,
    TIME,
    // TR,
    // TRO, Skipped.
}

impl AsRepr for ContractInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            // ContractInstruction::BAL => todo!(),
            ContractInstruction::BHEI => vec![op::bhei(0x10), op::jmpb(RegId::ZERO, 0)],
            ContractInstruction::BHSH => vec![
                op::movi(0x10, Bytes32::LEN.try_into().unwrap()),
                op::aloc(0x10),
                op::move_(0x10, RegId::HP),
                op::bhsh(0x10, RegId::ZERO),
                op::jmpb(RegId::ZERO, 0),
            ],
            // ContractInstruction::BURN => todo!(),
            // ContractInstruction::CALL => todo!(),
            ContractInstruction::CB => vec![
                op::movi(0x10, Bytes32::LEN.try_into().unwrap()),
                op::aloc(0x10),
                op::move_(0x10, RegId::HP),
                op::cb(0x10),
                op::jmpb(RegId::ZERO, 0),
            ],
            // ContractInstruction::CCP => todo!(),
            // ContractInstruction::CROO => todo!(),
            // ContractInstruction::CSIZ => todo!(),
            // ContractInstruction::LDC => todo!(),
            ContractInstruction::LOG => {
                vec![op::log(0x10, 0x11, 0x12, 0x13), op::jmpb(RegId::ZERO, 0)]
            }
            // ContractInstruction::LOGD => todo!(),
            // ContractInstruction::MINT => todo!(),
            // ContractInstruction::RETD => todo!(),
            // ContractInstruction::SMO => todo!(),
            // ContractInstruction::SCWQ => todo!(),
            // ContractInstruction::SRW => todo!(),
            // ContractInstruction::SRWQ => todo!(),
            // ContractInstruction::SWW => todo!(),
            // ContractInstruction::SWWQ => todo!(),
            ContractInstruction::TIME => vec![
                op::movi(0x10, 0),
                op::time(0x11, 0x10),
                op::jmpb(RegId::ZERO, 0),
            ],
            // ContractInstruction::TR => todo!(),
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        None
    }

    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        todo!()
    }

    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        todo!()
    }
}

impl ContractInstruction {
    pub fn contract_data(&self) -> Option<Vec<u8>> {
        None
    }
}
