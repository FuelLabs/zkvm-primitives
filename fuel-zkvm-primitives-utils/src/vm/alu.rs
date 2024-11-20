use fuel_core_types::fuel_asm::{op, RegId};

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
// - [ ] **WDCM**: 128-bit integer comparison
// - [ ] **WQCM**: 256-bit integer comparison
// - [ ] **WDOP**: Misc 128-bit integer operations
// - [ ] **WQOP**: Misc 256-bit integer operations
// - [ ] **WDML**: Multiply 128-bit integers
// - [ ] **WQML**: Multiply 256-bit integers
// - [ ] **WDDV**: 128-bit integer division
// - [ ] **WQDV**: 256-bit integer division
// - [ ] **WDMD**: 128-bit integer fused multiply-divide
// - [ ] **WQMD**: 256-bit integer fused multiply-divide
// - [ ] **WDAM**: Modular 128-bit integer addition
// - [ ] **WQAM**: Modular 256-bit integer addition
// - [ ] **WDMM**: Modular 128-bit integer multiplication
// - [ ] **WQMM**: Modular 256-bit integer multiplication
// - [ ] **XOR**: XOR
// - [ ] **XORI**: XOR immediate

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
