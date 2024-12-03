mod utils;

use crate::vm::base::AsRepr;
use crate::vm::contract::utils::{
    call_contract_once, call_contract_repeat, script_data, setup_instructions, u256_iterator_loop,
    u256_iterator_loop_with_step,
};
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuel_core_types::fuel_types::Bytes32;
use fuels::prelude::AssetId;
use fuels::prelude::ContractId;
use fuels::types::input::Input as TxInput;
use fuels::types::output::Output as TxOutput;
use std::str::FromStr;
use std::sync::OnceLock;

const ARBITRARY_INPUT: u32 = 10_000;

struct ContractInstructionMetadata {
    contract_metadata: ContractMetadata,
    #[allow(unused)]
    asset_id: AssetId,
    script_data: Vec<u8>,
    input: TxInput,
    output: TxOutput,
}

impl ContractInstructionMetadata {
    fn default_with_bytecode(contract_bytecode: Vec<Instruction>) -> Self {
        // if you set this to 0, IT WILL BREAK
        let contract_id = ContractId::from_str(
            "0xdeadbeef3d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1fdeadbeef",
        )
        .unwrap();
        let asset_id = AssetId::zeroed();

        let input = TxInput::Contract {
            utxo_id: Default::default(),
            balance_root: Bytes32::zeroed(),
            state_root: Bytes32::zeroed(),
            tx_pointer: Default::default(),
            contract_id,
        };

        let output = TxOutput::contract(0, Bytes32::zeroed(), Bytes32::zeroed());

        let script_data = script_data(&contract_id, &asset_id);

        let contract_metadata = ContractMetadata {
            contract_id,
            contract_bytecode: contract_bytecode.into_iter().collect(),
            state_size: 100_000,
        };

        ContractInstructionMetadata {
            contract_metadata,
            asset_id,
            script_data,
            input,
            output,
        }
    }
}

static BAL_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn bal_metadata() -> &'static ContractInstructionMetadata {
    BAL_METADATA
        .get_or_init(|| ContractInstructionMetadata::default_with_bytecode(vec![op::noop()]))
}

static BURN_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn burn_metadata() -> &'static ContractInstructionMetadata {
    BURN_METADATA.get_or_init(|| {
        ContractInstructionMetadata::default_with_bytecode(u256_iterator_loop(|iterator| {
            op::burn(RegId::ONE, iterator)
        }))
    })
}

static CCP_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn ccp_metadata() -> &'static ContractInstructionMetadata {
    CCP_METADATA.get_or_init(|| {
        let contract_bytecode = std::iter::repeat(op::noop())
            .take(ARBITRARY_INPUT as usize)
            .chain(vec![op::ret(RegId::ZERO)])
            .collect();

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static CROO_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn croo_metadata() -> &'static ContractInstructionMetadata {
    CROO_METADATA.get_or_init(|| {
        let contract_bytecode = vec![op::noop(); ARBITRARY_INPUT as usize];

        let mut metadata = ContractInstructionMetadata::default_with_bytecode(contract_bytecode);
        metadata.script_data = metadata
            .contract_metadata
            .contract_id
            .iter()
            .copied()
            .collect();
        metadata
    })
}

static CSIZ_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn csiz_metadata() -> &'static ContractInstructionMetadata {
    CSIZ_METADATA.get_or_init(|| {
        let contract_bytecode = std::iter::repeat(op::noop())
            .take(ARBITRARY_INPUT as usize)
            .chain(vec![op::ret(RegId::ZERO)])
            .collect();

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static LDC_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn ldc_metadata() -> &'static ContractInstructionMetadata {
    LDC_METADATA.get_or_init(|| {
        let contract_bytecode = std::iter::repeat(op::noop())
            .take(ARBITRARY_INPUT as usize)
            .chain(vec![op::ret(RegId::ZERO)])
            .collect();

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static MINT_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn mint_metadata() -> &'static ContractInstructionMetadata {
    MINT_METADATA.get_or_init(|| {
        ContractInstructionMetadata::default_with_bytecode(u256_iterator_loop(|iterator| {
            op::mint(RegId::ONE, iterator)
        }))
    })
}

static RETD_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn retd_metadata() -> &'static ContractInstructionMetadata {
    RETD_METADATA.get_or_init(|| {
        let contract_bytecode = vec![op::movi(0x14, ARBITRARY_INPUT), op::retd(RegId::ONE, 0x14)];

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static TR_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn tr_metadata() -> &'static ContractInstructionMetadata {
    TR_METADATA.get_or_init(|| {
        let contract_bytecode = u256_iterator_loop(|iterator| op::tr(0x15, 0x14, iterator));

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static SWW_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn sww_metadata() -> &'static ContractInstructionMetadata {
    SWW_METADATA.get_or_init(|| {
        let contract_bytecode = u256_iterator_loop(|iterator| op::sww(iterator, 0x29, RegId::ONE));

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static SWWQ_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn swwq_metadata() -> &'static ContractInstructionMetadata {
    SWWQ_METADATA.get_or_init(|| {
        let contract_bytecode = u256_iterator_loop_with_step(
            |iterator| op::swwq(iterator, 0x13, RegId::ZERO, 0x15),
            10,
        );

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static SRW_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn srw_metadata() -> &'static ContractInstructionMetadata {
    SRW_METADATA.get_or_init(|| {
        let contract_bytecode = u256_iterator_loop(|iterator| op::srw(0x13, 0x14, iterator));

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
    })
}

static SRWQ_METADATA: OnceLock<ContractInstructionMetadata> = OnceLock::new();

fn srwq_metadata() -> &'static ContractInstructionMetadata {
    SRWQ_METADATA.get_or_init(|| {
        let step = 10;
        let mut contract_bytecode = vec![
            op::movi(0x15, step),
            op::movi(0x16, step * Bytes32::LEN as u32),
            op::aloc(0x16),
            op::move_(0x17, RegId::HP),
        ];
        contract_bytecode.extend(u256_iterator_loop_with_step(
            |iterator| op::srwq(0x17, 0x13, iterator, 0x15),
            step,
        ));

        ContractInstructionMetadata::default_with_bytecode(contract_bytecode)
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
    BURN,
    // CALL,
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
    // SMO,
    // SCWQ,
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
            ContractInstruction::BAL => bal(),
            ContractInstruction::BHEI => vec![op::bhei(0x10), op::jmpb(RegId::ZERO, 0)],
            ContractInstruction::BHSH => vec![
                op::movi(0x10, Bytes32::LEN.try_into().unwrap()),
                op::aloc(0x10),
                op::move_(0x10, RegId::HP),
                op::bhsh(0x10, RegId::ZERO),
                op::jmpb(RegId::ZERO, 0),
            ],
            ContractInstruction::BURN => burn(),
            // ContractInstruction::CALL => todo!(),
            ContractInstruction::CB => vec![
                op::movi(0x10, Bytes32::LEN.try_into().unwrap()),
                op::aloc(0x10),
                op::move_(0x10, RegId::HP),
                op::cb(0x10),
                op::jmpb(RegId::ZERO, 0),
            ],
            ContractInstruction::CCP => ccp(),
            ContractInstruction::CROO => croo(),
            ContractInstruction::CSIZ => csiz(),
            ContractInstruction::LDC => ldc(),
            ContractInstruction::LOG => {
                vec![op::log(0x10, 0x11, 0x12, 0x13), op::jmpb(RegId::ZERO, 0)]
            }
            ContractInstruction::LOGD => logd(),
            ContractInstruction::MINT => mint(),
            ContractInstruction::RETD => retd(),
            // ContractInstruction::SMO => todo!(),
            // ContractInstruction::SCWQ => todo!(),
            ContractInstruction::SRW => srw(),
            ContractInstruction::SRWQ => srwq(),
            ContractInstruction::SWW => sww(),
            ContractInstruction::SWWQ => swwq(),
            ContractInstruction::TIME => vec![
                op::movi(0x10, 0),
                op::time(0x11, 0x10),
                op::jmpb(RegId::ZERO, 0),
            ],
            ContractInstruction::TR => tr(),
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        match &self {
            ContractInstruction::BAL => Some(bal_metadata().script_data.clone()),
            ContractInstruction::BURN => Some(burn_metadata().script_data.clone()),
            ContractInstruction::CCP => Some(ccp_metadata().script_data.clone()),
            ContractInstruction::CROO => Some(croo_metadata().script_data.clone()),
            ContractInstruction::CSIZ => Some(csiz_metadata().script_data.clone()),
            ContractInstruction::LDC => Some(ldc_metadata().script_data.clone()),
            ContractInstruction::LOGD => {
                Some(script_data(&ContractId::zeroed(), &AssetId::zeroed()))
            }
            ContractInstruction::MINT => Some(mint_metadata().script_data.clone()),
            ContractInstruction::RETD => Some(retd_metadata().script_data.clone()),
            ContractInstruction::TR => Some(tr_metadata().script_data.clone()),
            ContractInstruction::SWW => Some(sww_metadata().script_data.clone()),
            ContractInstruction::SWWQ => Some(swwq_metadata().script_data.clone()),
            ContractInstruction::SRW => Some(srw_metadata().script_data.clone()),
            ContractInstruction::SRWQ => Some(srwq_metadata().script_data.clone()),
            _ => None,
        }
    }

    fn additional_inputs(&self) -> Option<Vec<TxInput>> {
        match &self {
            ContractInstruction::BAL => Some(vec![bal_metadata().input.clone()]),
            ContractInstruction::BURN => Some(vec![burn_metadata().input.clone()]),
            ContractInstruction::CCP => Some(vec![ccp_metadata().input.clone()]),
            ContractInstruction::CROO => Some(vec![croo_metadata().input.clone()]),
            ContractInstruction::CSIZ => Some(vec![csiz_metadata().input.clone()]),
            ContractInstruction::LDC => Some(vec![ldc_metadata().input.clone()]),
            ContractInstruction::MINT => Some(vec![mint_metadata().input.clone()]),
            ContractInstruction::RETD => Some(vec![retd_metadata().input.clone()]),
            ContractInstruction::TR => Some(vec![tr_metadata().input.clone()]),
            ContractInstruction::SWW => Some(vec![sww_metadata().input.clone()]),
            ContractInstruction::SWWQ => Some(vec![swwq_metadata().input.clone()]),
            ContractInstruction::SRW => Some(vec![srw_metadata().input.clone()]),
            ContractInstruction::SRWQ => Some(vec![srwq_metadata().input.clone()]),
            _ => None,
        }
    }

    fn additional_outputs(&self) -> Option<Vec<TxOutput>> {
        match &self {
            ContractInstruction::BAL => Some(vec![bal_metadata().output]),
            ContractInstruction::BURN => Some(vec![burn_metadata().output]),
            ContractInstruction::CCP => Some(vec![ccp_metadata().output]),
            ContractInstruction::CROO => Some(vec![croo_metadata().output]),
            ContractInstruction::CSIZ => Some(vec![csiz_metadata().output]),
            ContractInstruction::LDC => Some(vec![ldc_metadata().output]),
            ContractInstruction::MINT => Some(vec![mint_metadata().output]),
            ContractInstruction::RETD => Some(vec![retd_metadata().output]),
            ContractInstruction::TR => Some(vec![tr_metadata().output]),
            ContractInstruction::SWW => Some(vec![sww_metadata().output]),
            ContractInstruction::SWWQ => Some(vec![swwq_metadata().output]),
            ContractInstruction::SRW => Some(vec![srw_metadata().output]),
            ContractInstruction::SRWQ => Some(vec![srwq_metadata().output]),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub struct ContractMetadata {
    pub contract_id: ContractId,
    pub contract_bytecode: Vec<u8>,
    pub state_size: usize,
}

impl ContractInstruction {
    pub fn contract_metadata(&self) -> Option<ContractMetadata> {
        match &self {
            ContractInstruction::BAL => Some(bal_metadata().contract_metadata.clone()),
            ContractInstruction::BURN => Some(burn_metadata().contract_metadata.clone()),
            ContractInstruction::CCP => Some(ccp_metadata().contract_metadata.clone()),
            ContractInstruction::CROO => Some(croo_metadata().contract_metadata.clone()),
            ContractInstruction::CSIZ => Some(csiz_metadata().contract_metadata.clone()),
            ContractInstruction::LDC => Some(ldc_metadata().contract_metadata.clone()),
            ContractInstruction::MINT => Some(mint_metadata().contract_metadata.clone()),
            ContractInstruction::RETD => Some(retd_metadata().contract_metadata.clone()),
            ContractInstruction::TR => Some(tr_metadata().contract_metadata.clone()),
            ContractInstruction::SWW => Some(sww_metadata().contract_metadata.clone()),
            ContractInstruction::SWWQ => Some(swwq_metadata().contract_metadata.clone()),
            ContractInstruction::SRW => Some(srw_metadata().contract_metadata.clone()),
            ContractInstruction::SRWQ => Some(srwq_metadata().contract_metadata.clone()),
            _ => None,
        }
    }
}

fn bal() -> Vec<Instruction> {
    vec![
        op::gtf_args(0x11, 0x00, GTFArgs::ScriptData),
        op::addi(0x12, 0x11, ContractId::LEN.try_into().unwrap()),
        op::bal(0x10, 0x12, 0x11),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn burn() -> Vec<Instruction> {
    call_contract_once()
}

fn ccp() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::movi(0x13, ARBITRARY_INPUT),
        op::movi(0x14, ARBITRARY_INPUT),
        op::movi(0x15, ARBITRARY_INPUT),
        op::add(0x15, 0x15, 0x15),
        op::addi(0x15, 0x15, 32),
        op::aloc(0x15),
        op::move_(0x15, RegId::HP),
        op::ccp(0x15, 0x10, RegId::ZERO, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    instructions
}

fn croo() -> Vec<Instruction> {
    vec![
        op::gtf_args(0x16, 0x00, GTFArgs::ScriptData),
        op::movi(0x15, 2000),
        op::aloc(0x15),
        op::move_(0x14, RegId::HP),
        op::croo(0x14, 0x16),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn csiz() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
        op::csiz(0x11, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]);

    instructions
}

fn ldc() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::movi(0x13, ARBITRARY_INPUT),
        op::ldc(0x10, RegId::ZERO, 0x13, 0),
        op::jmpb(RegId::ZERO, 0),
    ]);

    instructions
}

fn logd() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::movi(0x13, ARBITRARY_INPUT),
        op::logd(0x10, 0x11, RegId::ZERO, 0x13),
        op::jmpb(RegId::ZERO, 0),
    ]);

    instructions
}

fn mint() -> Vec<Instruction> {
    call_contract_once()
}

fn retd() -> Vec<Instruction> {
    call_contract_repeat()
}

fn tr() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::movi(0x13, (1 << 18) - 1),
        op::movi(0x15, 2000),
        op::movi(0x14, 1),
        op::call(0x10, 0x13, 0x15, RegId::CGAS),
    ]);

    instructions
}

fn sww() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS)]);

    instructions
}

fn swwq() -> Vec<Instruction> {
    let mut instructions = vec![op::movi(0x15, 10)];
    instructions.extend(call_contract_once());
    instructions
}

fn srw() -> Vec<Instruction> {
    let mut instructions = setup_instructions();
    instructions.extend(vec![
        op::movi(0x15, 2000),
        op::call(0x10, RegId::ZERO, 0x11, RegId::CGAS),
    ]);

    instructions
}

fn srwq() -> Vec<Instruction> {
    call_contract_once()
}
