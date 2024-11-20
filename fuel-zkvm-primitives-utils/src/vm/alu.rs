use ethnum::U256;
use fuel_core_types::fuel_asm::wideint::{
    CompareArgs, CompareMode, DivArgs, MathArgs, MathOp, MulArgs,
};
use fuel_core_types::fuel_asm::{op, Instruction, RegId};

/// This file contains helpers to generate scripts with various alu operations in an infinite loop.
/// Below is a checklist of what has been implemented and what hasn't
// - [x] **ADD**: Add
// - [x] **ADDI**: Add immediate
// - [x] **AND**: AND
// - [x] **ANDI**: AND immediate
// - [x] **DIV**: Divide
// - [x] **DIVI**: Divide immediate
// - [x] **EQ**: Equals
// - [x] **EXP**: Exponentiate
// - [x] **EXPI**: Exponentiate immediate
// - [x] **GT**: Greater than
// - [x] **LT**: Less than
// - [x] **MLOG**: Math logarithm
// - [x] **MOD**: Modulus
// - [x] **MODI**: Modulus immediate
// - [x] **MOVE**: Move
// - [x] **MOVI**: Move immediate
// - [x] **MROO**: Math root
// - [x] **MUL**: Multiply
// - [x] **MULI**: Multiply immediate
// - [x] **MLDV**: Fused multiply-divide
// - [x] **NOOP**: No operation
// - [x] **NOT**: Invert
// - [x] **OR**: OR
// - [x] **ORI**: OR immediate
// - [x] **SLL**: Shift left logical
// - [x] **SLLI**: Shift left logical immediate
// - [x] **SRL**: Shift right logical
// - [x] **SRLI**: Shift right logical immediate
// - [x] **SUB**: Subtract
// - [x] **SUBI**: Subtract immediate
// - [x] **WDCM**: 128-bit integer comparison
// - [x] **WQCM**: 256-bit integer comparison
// - [x] **WDOP**: Misc 128-bit integer operations
// - [x] **WQOP**: Misc 256-bit integer operations
// - [x] **WDML**: Multiply 128-bit integers
// - [x] **WQML**: Multiply 256-bit integers
// - [x] **WDDV**: 128-bit integer division
// - [x] **WQDV**: 256-bit integer division
// - [x] **WDMD**: 128-bit integer fused multiply-divide
// - [x] **WQMD**: 256-bit integer fused multiply-divide
// - [x] **WDAM**: Modular 128-bit integer addition
// - [x] **WQAM**: Modular 256-bit integer addition
// - [x] **WDMM**: Modular 128-bit integer multiplication
// - [x] **WQMM**: Modular 256-bit integer multiplication
// - [x] **XOR**: XOR
// - [x] **XORI**: XOR immediate

pub fn add() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::add(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn addi() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::addi(0x11, 0x10, 1024),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn and() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::and(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn andi() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::andi(0x11, 0x10, 123),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn sub() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::sub(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn subi() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::subi(0x11, 0x10, 10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn div() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 4),
        op::div(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn divi() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::divi(0x11, 0x10, 4),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn eq() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 1024),
        op::eq(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn exp() -> Vec<u8> {
    [
        op::movi(0x10, 2),
        op::movi(0x11, 10),
        op::exp(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn expi() -> Vec<u8> {
    [
        op::movi(0x10, 2),
        op::expi(0x11, 0x10, 10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn gt() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::gt(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn lt() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::lt(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn mlog() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 10),
        op::mlog(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn mod_() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 10),
        op::mod_(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn modi() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::modi(0x11, 0x10, 10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn move_() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::move_(0x11, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn movi() -> Vec<u8> {
    [op::movi(0x10, 1024), op::jmpb(RegId::ZERO, 0)]
        .into_iter()
        .collect()
}

pub fn mroo() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 2),
        op::mroo(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn mul() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 3),
        op::muli(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn muli() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::muli(0x11, 0x10, 10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn mldv() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 5),
        op::movi(0x12, 10),
        op::mldv(0x13, 0x10, 0x11, 0x12),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn noop() -> Vec<u8> {
    [op::noop(), op::jmpb(RegId::ZERO, 0)].into_iter().collect()
}

pub fn not() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::not(0x11, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn or() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::or(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn ori() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::ori(0x11, 0x10, 123),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn sll() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::sll(0x11, 0x10, 2),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn slli() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::slli(0x11, 0x10, 2),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn srl() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::srl(0x11, 0x10, 2),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn srli() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::srli(0x11, 0x10, 2),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

/// Copied from https://github.com/FuelLabs/fuel-core/blob/4986d4d034499dafc19b9dcd72458717b6ecdd5b/benches/benches/utils.rs#L38-L57
/// Allocates a byte array from heap and initializes it. Then points `reg` to it.
fn alloc_bytearray<const S: usize>(reg: u8, v: [u8; S]) -> Vec<Instruction> {
    let mut ops = vec![op::movi(reg, S as u32), op::aloc(reg)];
    for (i, b) in v.iter().enumerate() {
        if *b != 0 {
            ops.push(op::movi(reg, *b as u32));
            ops.push(op::sb(RegId::HP, reg, i as u16));
        }
    }
    ops.push(op::move_(reg, RegId::HP));
    ops
}

fn make_u128(reg: u8, v: u128) -> Vec<Instruction> {
    alloc_bytearray(reg, v.to_be_bytes())
}

fn make_u256(reg: u8, v: ethnum::U256) -> Vec<Instruction> {
    alloc_bytearray(reg, v.to_be_bytes())
}

fn prepared_wideint_u128() -> Vec<Instruction> {
    let mut wideint_prepare = Vec::new();
    wideint_prepare.extend(make_u128(0x10, 0));
    wideint_prepare.extend(make_u128(0x11, u128::MAX));
    wideint_prepare.extend(make_u128(0x12, u128::MAX / 2 + 1));
    wideint_prepare.extend(make_u128(0x13, u128::MAX - 158)); // prime
    wideint_prepare.extend(make_u128(0x14, u64::MAX.into()));

    wideint_prepare
}

pub fn wdcm() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdcm_args(
            0x10,
            0x12,
            0x13,
            CompareArgs {
                mode: CompareMode::LTE,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wdop() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdop_args(
            0x10,
            0x13,
            0x12,
            MathArgs {
                op: MathOp::SUB,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wdml() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdml_args(
            0x10,
            0x14,
            0x14,
            MulArgs {
                indirect_lhs: true,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wddv() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wddv_args(0x10, 0x12, 0x13, DivArgs { indirect_rhs: true }),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wdmd() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdmd(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wdam() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdam(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wdmm() -> Vec<u8> {
    let mut harness = prepared_wideint_u128();
    harness.extend(vec![
        op::wdmm(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

fn prepared_wideint_u256() -> Vec<Instruction> {
    let mut wideint_prepare = Vec::new();
    wideint_prepare.extend(make_u256(0x10, U256::ZERO));
    wideint_prepare.extend(make_u256(0x11, U256::MAX));
    wideint_prepare.extend(make_u256(0x12, U256::MAX / 2 + 1));
    wideint_prepare.extend(make_u256(0x13, U256::MAX - 188)); // prime
    wideint_prepare.extend(make_u256(0x14, u128::MAX.into()));

    wideint_prepare
}

pub fn wqcm() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();

    harness.extend(vec![
        op::wqcm_args(
            0x10,
            0x12,
            0x13,
            CompareArgs {
                mode: CompareMode::LTE,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqop() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqop_args(
            0x10,
            0x13,
            0x12,
            MathArgs {
                op: MathOp::SUB,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqml() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqml_args(
            0x10,
            0x14,
            0x14,
            MulArgs {
                indirect_lhs: true,
                indirect_rhs: true,
            },
        ),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqdv() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqdv_args(0x10, 0x12, 0x13, DivArgs { indirect_rhs: true }),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqmd() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqmd(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqam() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqam(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn wqmm() -> Vec<u8> {
    let mut harness = prepared_wideint_u256();
    harness.extend(vec![
        op::wqmm(0x10, 0x12, 0x13, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    harness.into_iter().collect()
}

pub fn xor() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::movi(0x11, 123),
        op::xor(0x12, 0x10, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}

pub fn xori() -> Vec<u8> {
    [
        op::movi(0x10, 1024),
        op::xori(0x11, 0x10, 123),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect()
}
