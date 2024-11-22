use crate::vm::AsRepr;
use fuel_core_types::fuel_asm::{op, Instruction, RegId};
use fuel_core_types::fuel_tx::Word;
use fuel_core_types::fuel_types::RegisterId;

// all fixtures obtained from https://github.com/FuelLabs/fuel-core/blob/62766787f9e24f9e581dcaada9dfa982355ea89f/benches/benches/block_target_gas_set/memory.rs

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug)]
pub enum MemoryInstruction {
    ALOC,
    CFE,
    CFEI,
    CFS,
    CFSI,
    LB,
    LW,
    MCL,
    MCLI,
    MCP,
    MCPI,
    MEQ,
    POPH,
    POPL,
    PSHH,
    PSHL,
    SB,
    SW,
}

const U32_MASK: u32 = (1 << 24) - 1;

impl AsRepr for MemoryInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            MemoryInstruction::ALOC => {
                vec![op::movi(0x10, 0), op::aloc(0x10), op::jmpb(RegId::ZERO, 0)]
            }
            MemoryInstruction::CFE => cfe(),
            MemoryInstruction::CFEI => cfei(),
            MemoryInstruction::CFS => cfs(),
            MemoryInstruction::CFSI => cfsi(),
            MemoryInstruction::LB => vec![op::lb(0x10, RegId::ONE, 10), op::jmpb(RegId::ZERO, 0)],
            MemoryInstruction::LW => vec![op::lw(0x10, RegId::ONE, 10), op::jmpb(RegId::ZERO, 0)],
            MemoryInstruction::MCL => vec![
                op::movi(0x11, 100000),
                op::aloc(0x11),
                op::move_(0x10, RegId::HP),
                op::mcl(0x10, 0x11),
                op::jmpb(RegId::ZERO, 0),
            ],
            MemoryInstruction::MCLI => vec![
                op::movi(0x11, 100000),
                op::aloc(0x11),
                op::move_(0x10, RegId::HP),
                op::mcli(0x10, 100000),
                op::jmpb(RegId::ZERO, 0),
            ],
            MemoryInstruction::MCP => vec![
                op::movi(0x11, 100000),
                op::aloc(0x11),
                op::move_(0x10, RegId::HP),
                op::mcp(0x10, RegId::ZERO, 0x11),
                op::jmpb(RegId::ZERO, 0),
            ],
            MemoryInstruction::MCPI => vec![
                op::movi(0x11, 1000),
                op::aloc(0x11),
                op::move_(0x10, RegId::HP),
                op::mcpi(0x10, RegId::ZERO, 1000),
                op::jmpb(RegId::ZERO, 0),
            ],
            MemoryInstruction::MEQ => meq(),
            MemoryInstruction::POPH => vec![
                op::pshh(U32_MASK),
                op::poph(U32_MASK),
                op::jmpb(RegId::ZERO, 1),
            ],
            MemoryInstruction::POPL => vec![
                op::pshl(U32_MASK),
                op::popl(U32_MASK),
                op::jmpb(RegId::ZERO, 1),
            ],
            MemoryInstruction::PSHH => vec![op::pshh(U32_MASK), op::jmpb(RegId::ZERO, 0)],
            MemoryInstruction::PSHL => vec![op::pshl(U32_MASK), op::jmpb(RegId::ZERO, 0)],
            MemoryInstruction::SB => vec![
                op::aloc(RegId::ONE),
                op::move_(0x10, RegId::HP),
                op::movi(0x11, 50),
                op::sb(0x10, 0x11, 0),
                op::jmpb(RegId::ZERO, 0),
            ],
            MemoryInstruction::SW => vec![
                op::movi(0x10, 8),
                op::aloc(0x10),
                op::move_(0x10, RegId::HP),
                op::movi(0x11, 50),
                op::sw(0x10, 0x11, 0),
                op::jmpb(RegId::ZERO, 0),
            ],
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        None
    }
}

fn cfe() -> Vec<Instruction> {
    vec![
        op::movi(0x10, 10),
        op::movi(0x11, 100),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfe(0x10),
        op::cfs(0x11),
        op::jmpb(RegId::ZERO, 10),
    ]
}

fn cfei() -> Vec<Instruction> {
    vec![
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfei(10),
        op::cfsi(100),
        op::jmpb(RegId::ZERO, 10),
    ]
}

fn cfs() -> Vec<Instruction> {
    vec![
        op::movi(0x10, 100),
        op::movi(0x11, 10),
        op::cfe(0x10),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::cfs(0x11),
        op::jmpb(RegId::ZERO, 10),
    ]
}

fn cfsi() -> Vec<Instruction> {
    vec![
        op::cfei(100),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::cfsi(10),
        op::jmpb(RegId::ZERO, 10),
    ]
}

pub fn set_full_word(r: RegisterId, v: Word) -> Vec<Instruction> {
    let r = u8::try_from(r).unwrap();
    let mut ops = vec![op::movi(r, 0)];
    for byte in v.to_be_bytes() {
        ops.push(op::ori(r, r, byte as u16));
        ops.push(op::slli(r, r, 8));
    }
    ops.pop().unwrap(); // Remove last shift
    ops
}

fn meq() -> Vec<Instruction> {
    let mut prepared = set_full_word(0x13, 100000);
    prepared.extend(vec![
        op::meq(0x10, RegId::ZERO, RegId::ZERO, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);
    prepared
}
