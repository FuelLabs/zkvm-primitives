//! copied from https://github.com/FuelLabs/fuel-core/blob/328b42ca5d26f7ca74b979df38137e001b5a86ed/benches/benches/block_target_gas.rs

use fuel_core_types::fuel_asm::wideint::MathOp;
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuel_core_types::fuel_tx::{AssetId, ContractId};
use fuel_core_types::fuel_vm::consts::WORD_SIZE;

pub fn call_contract_once() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS)]);
    instructions
}

pub fn call_contract_repeat() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS),
        op::jmpb(RegId::ZERO, 0),
    ]);
    instructions
}

/// Returns a bytecode that contains an infinite loop that increases the `u256` iterator by
/// `1` each iteration. A function expects a closure that returns an opcode that must
/// be called infinitely. The closure should accept one argument -
/// the register where the iterator is stored.
pub fn u256_iterator_loop(opcode: impl Fn(RegId) -> Instruction) -> Vec<Instruction> {
    u256_iterator_loop_with_step(opcode, 1)
}

/// Returns a bytecode that contains an infinite loop that increases the `u256` iterator by
/// `step` each iteration. A function expects a closure that returns an opcode that must
/// be called infinitely. The closure should accept one argument -
/// the register where the iterator is stored.
pub fn u256_iterator_loop_with_step(
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
        // We need to pad number of instructions to be 8-byte aligned.
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

pub fn setup_instructions() -> Vec<Instruction> {
    vec![
        op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
        op::addi(0x11, 0x10, ContractId::LEN.try_into().unwrap()),
        op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
        op::addi(0x11, 0x11, WORD_SIZE.try_into().unwrap()),
        op::movi(0x12, (1 << 18) - 1),
    ]
}

pub fn script_data(contract_id: &ContractId, asset_id: &AssetId) -> Vec<u8> {
    contract_id
        .iter()
        .copied()
        .chain(
            (0 as fuel_core_types::fuel_asm::Word)
                .to_be_bytes()
                .iter()
                .copied(),
        )
        .chain(
            (0 as fuel_core_types::fuel_asm::Word)
                .to_be_bytes()
                .iter()
                .copied(),
        )
        .chain(asset_id.iter().copied())
        .collect()
}
