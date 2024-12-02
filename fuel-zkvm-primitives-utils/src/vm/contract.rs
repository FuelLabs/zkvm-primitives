use crate::vm::base::AsRepr;
use fuel_core_types::fuel_asm::{op, Instruction, RegId};
use fuel_core_types::fuel_tx::Word;
use fuel_core_types::fuel_types::bytes::WORD_SIZE;
use fuel_core_types::fuel_types::Bytes32;
use fuels::prelude::AssetId;
use fuels::prelude::ContractId;
use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;
use std::sync::OnceLock;

struct ContractInstructionMetadata {
    contract_id: ContractId,
    #[allow(unused)]
    asset_id: AssetId,
    script_data: Vec<u8>,
    input: TxInput,
    output: TxOutput,
    bytecode: Vec<u8>,
}

static BAL_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn bal_metadata() -> &'static ContractInstructionMetadata {
    BAL_METADATA.get_or_init(|| {
        let contract_id = ContractId::zeroed();
        let asset_id = AssetId::zeroed();

        let input = TxInput::Contract {
            utxo_id: Default::default(),
            balance_root: Bytes32::zeroed(),
            state_root: Bytes32::zeroed(),
            tx_pointer: Default::default(),
            contract_id,
        };

        let output = TxOutput::contract(0, Bytes32::zeroed(), Bytes32::zeroed());

        let script_data = contract_id
            .iter()
            .copied()
            .chain((0 as Word).to_be_bytes().iter().copied())
            .chain((0 as Word).to_be_bytes().iter().copied())
            .chain(asset_id.iter().copied())
            .collect();

        let bytecode = vec![0x10, 0x11, 0x12];

        ContractInstructionMetadata {
            contract_id,
            asset_id,
            script_data,
            input,
            output,
            bytecode,
        }
    })
}

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum ContractInstruction {
    BAL,
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
            ContractInstruction::BAL => bal(),
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
        match &self {
            ContractInstruction::BAL => Some(bal_metadata().script_data.clone()),
            _ => None,
        }
    }

    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        match &self {
            ContractInstruction::BAL => Some(vec![bal_metadata().input.clone()]),
            _ => None,
        }
    }

    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        match &self {
            ContractInstruction::BAL => Some(vec![bal_metadata().output.clone()]),
            _ => None,
        }
    }
}

impl ContractInstruction {
    pub fn contract_metadata(&self) -> Option<(ContractId, Vec<u8>)> {
        match &self {
            ContractInstruction::BAL => {
                Some((bal_metadata().contract_id, bal_metadata().bytecode.clone()))
            }
            _ => None,
        }
    }
}

fn bal() -> Vec<Instruction> {
    let word_size = WORD_SIZE.try_into().unwrap();
    vec![
        // alloc 32 empty bytes
        op::movi(0x13, word_size),
        op::aloc(0x13),
        // alloc 32 empty bytes
        op::movi(0x14, word_size),
        op::aloc(0x14),
        // first memory address for asset id
        op::movi(0x11, 0),
        // second memory address for contract id
        op::addi(0x12, 0x11, word_size as u16),
        op::bal(0x10, 0x11, 0x12),
        op::jmpb(RegId::ZERO, 0),
    ]
}
