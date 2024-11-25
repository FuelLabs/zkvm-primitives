use crate::vm::base::AsRepr;
use fuel_core_types::fuel_asm::wideint::MathOp;
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuel_core_types::fuel_tx::ContractId;
use fuel_core_types::fuel_vm::consts::WORD_SIZE;

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[derive(Debug, Copy, Clone)]
pub enum ContractInstruction {
    BAL,
    BHEI,
    BHSH,
    BURN,
    CALL,
    CB,
    CCP,
    CROO,
    CSIZ,
    LDC,
    LOG,
    LOGD,
    MINT,
    RETD,
    // RVRT, Skipped.
    SMO,
    SCWQ,
    SRW,
    SRWQ,
    SWW,
    SWWQ,
    TIME,
    TR,
    // TRO, Skipped.
}

impl AsRepr for ContractInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            ContractInstruction::BAL => todo!(),
            ContractInstruction::BHEI => todo!(),
            ContractInstruction::BHSH => todo!(),
            ContractInstruction::BURN => todo!(),
            ContractInstruction::CALL => todo!(),
            ContractInstruction::CB => todo!(),
            ContractInstruction::CCP => todo!(),
            ContractInstruction::CROO => todo!(),
            ContractInstruction::CSIZ => todo!(),
            ContractInstruction::LDC => todo!(),
            ContractInstruction::LOG => todo!(),
            ContractInstruction::LOGD => todo!(),
            ContractInstruction::MINT => todo!(),
            ContractInstruction::RETD => todo!(),
            ContractInstruction::SMO => todo!(),
            ContractInstruction::SCWQ => todo!(),
            ContractInstruction::SRW => todo!(),
            ContractInstruction::SRWQ => todo!(),
            ContractInstruction::SWW => todo!(),
            ContractInstruction::SWWQ => todo!(),
            ContractInstruction::TIME => todo!(),
            ContractInstruction::TR => todo!(),
        }
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        None
    }
}

impl ContractInstruction {
    pub fn contract_data(&self) -> Vec<u8> {
        match &self {
            ContractInstruction::BAL => {
                u256_iterator_loop(|iterator| op::bal(0x13, iterator, 0x10))
            }
            ContractInstruction::BHEI => todo!(),
            ContractInstruction::BHSH => todo!(),
            ContractInstruction::BURN => todo!(),
            ContractInstruction::CALL => todo!(),
            ContractInstruction::CB => todo!(),
            ContractInstruction::CCP => todo!(),
            ContractInstruction::CROO => todo!(),
            ContractInstruction::CSIZ => todo!(),
            ContractInstruction::LDC => todo!(),
            ContractInstruction::LOG => todo!(),
            ContractInstruction::LOGD => todo!(),
            ContractInstruction::MINT => todo!(),
            ContractInstruction::RETD => todo!(),
            ContractInstruction::SMO => todo!(),
            ContractInstruction::SCWQ => todo!(),
            ContractInstruction::SRW => todo!(),
            ContractInstruction::SRWQ => todo!(),
            ContractInstruction::SWW => todo!(),
            ContractInstruction::SWWQ => todo!(),
            ContractInstruction::TIME => todo!(),
            ContractInstruction::TR => todo!(),
        }
        .into_iter()
        .collect()
    }
}

// Copied from https://github.com/FuelLabs/fuel-core/blob/328b42ca5d26f7ca74b979df38137e001b5a86ed/benches/benches/block_target_gas.rs#L514

fn u256_iterator_loop(opcode: impl Fn(RegId) -> Instruction) -> Vec<Instruction> {
    u256_iterator_loop_with_step(opcode, 1)
}

fn u256_iterator_loop_with_step(
    opcode: impl Fn(RegId) -> Instruction,
    step: u32,
) -> Vec<Instruction> {
    // Register where we store an iterator.
    let iterator_register = RegId::new(0x20);
    let step_register = RegId::new(0x21);
    vec![
        // Store size of the iterator.
        op::movi(iterator_register, 32),
        // Store step value.
        op::movi(step_register, step),
        // Allocate 32 bytes for u256 iterator.
        op::aloc(iterator_register),
        // Store the address of the u256 iterator into `iterator_register`.
        op::move_(iterator_register, RegId::HP),
        // We need to pad number of isntruciton to be 8-byte aligned.
        op::noop(),
        // Execute benchmarking opcode.
        opcode(iterator_register),
        // Increment the iterator by one.
        op::wqop(
            iterator_register,
            iterator_register,
            step_register,
            MathOp::ADD as u8,
        ),
        // Jump 4 instructions(jmpb, wqop, opcode, noop) back.
        op::jmpb(RegId::ZERO, 1),
    ]
}

fn setup_instructions() -> Vec<Instruction> {
    vec![
        op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
        op::addi(0x11, 0x10, ContractId::LEN.try_into().unwrap()),
        op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
        op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
        op::movi(0x12, (1 << 18) - 1),
    ]
}

fn call_contract_repeat() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS),
        op::jmpb(RegId::ZERO, 0),
    ]);
    instructions
}

fn call_contract_once() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS)]);
    instructions
}
