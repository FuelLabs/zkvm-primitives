use crate::vm::AsRepr;
use fuel_core_types::fuel_asm::{op, GTFArgs, RegId};
use fuel_core_types::fuel_tx::{ContractId, Word};
use fuel_core_types::fuel_vm::consts::WORD_SIZE;
use fuels::prelude::Contract;
use fuels::types::AssetId;
use std::sync::OnceLock;

static GM_METADATA: OnceLock<(ContractId, AssetId, Vec<u8>, Vec<u8>)> = OnceLock::new();

fn gm_metadata() -> &'static (ContractId, AssetId, Vec<u8>, Vec<u8>) {
    GM_METADATA.get_or_init(|| {
        let contract_code = vec![op::gm(0x10, 1), op::jmpb(RegId::ZERO, 0)];
        let raw_contract_code: Vec<u8> = contract_code.clone().into_iter().collect();
        let contract = Contract::regular(
            raw_contract_code.clone(),
            Default::default(),
            Default::default(),
        );
        let contract_id = contract.contract_id();
        let asset_id = AssetId::zeroed();
        let script_data = contract_id
            .iter()
            .copied()
            .chain((0 as Word).to_be_bytes().iter().copied())
            .chain((0 as Word).to_be_bytes().iter().copied())
            .chain(asset_id.iter().copied())
            .collect();
        (contract_id, asset_id, raw_contract_code, script_data)
    })
}

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
                op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
                op::addi(0x11, 0x10, ContractId::LEN.try_into().unwrap()),
                op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
                op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
                op::movi(0x12, (1 << 18) - 1),
                op::call(0x10, RegId::ZERO, 0x11, 0x12),
            ],
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        match &self {
            OtherInstruction::GM => Some(gm_metadata().3.clone()),
            OtherInstruction::GTF => Some(vec![0x10, 0x11, 0x12, 0x13]),
            _ => None,
        }
    }
}

impl OtherInstruction {
    pub fn scaffold(&self) -> Vec<u8> {
        match &self {
            OtherInstruction::GM => gm_metadata().2.clone(),
            _ => vec![],
        }
        .into_iter()
        .collect()
    }
}
