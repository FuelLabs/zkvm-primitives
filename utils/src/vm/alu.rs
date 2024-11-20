use fuel_core_types::fuel_asm::{op, RegId};

/// This file contains helpers to generate scripts with various alu operations.
/// Below is a checklist of what has been implemented and what hasn't
// - [x] **ADD**: Add
// - [x] **ADDI**: Add immediate
// - [x] **AND**: AND
// - [x] **ANDI**: AND immediate
// - [x] **DIV**: Divide
// - [x] **DIVI**: Divide immediate
// - [ ] **EQ**: Equals
// - [ ] **EXP**: Exponentiate
// - [ ] **EXPI**: Exponentiate immediate
// - [ ] **GT**: Greater than
// - [ ] **LT**: Less than
// - [ ] **MLOG**: Math logarithm
// - [ ] **MOD**: Modulus
// - [ ] **MODI**: Modulus immediate
// - [ ] **MOVE**: Move
// - [ ] **MOVI**: Move immediate
// - [ ] **MROO**: Math root
// - [x] **MUL**: Multiply
// - [x] **MULI**: Multiply immediate
// - [ ] **MLDV**: Fused multiply-divide
// - [ ] **NOOP**: No operation
// - [ ] **NOT**: Invert
// - [ ] **OR**: OR
// - [ ] **ORI**: OR immediate
// - [ ] **SLL**: Shift left logical
// - [ ] **SLLI**: Shift left logical immediate
// - [ ] **SRL**: Shift right logical
// - [ ] **SRLI**: Shift right logical immediate
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
