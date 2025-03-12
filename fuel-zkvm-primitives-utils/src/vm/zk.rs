use crate::vm::base::AsRepr;
use fuel_core_types::fuel_asm::{op, GTFArgs, RegId};
use std::sync::OnceLock;

/// This file contains helpers to generate scripts with various zk operations in an infinite loop.

#[allow(non_camel_case_types)]
#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Clone)]
pub enum ZkInstruction {
    ECOP_ALT_BN_128,
    EPAR_ALT_BN_128,
}

struct ZkInstructionMetadata {
    script_data: Vec<u8>,
    script: Vec<u8>,
}

static ECOP_ALT_BN_128_METADATA: OnceLock<ZkInstructionMetadata> = OnceLock::new();

fn ecop_alt_bn_128_metadata() -> &'static ZkInstructionMetadata {
    let mut points_bytearray = Vec::with_capacity(3);
    // X
    points_bytearray.extend(
        hex::decode("2bd3e6d0f3b142924f5ca7b49ce5b9d54c4703d7ae5648e61d02268b1a0a9fb7").unwrap(),
    );

    // Y
    points_bytearray.extend(
        hex::decode("21611ce0a6af85915e2f1d70300909ce2e49dfad4a4619c8390cae66cefdb204").unwrap(),
    );

    // Scalar
    points_bytearray.extend(
        hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap(),
    );

    // total 96 bytes
    let script_reg_id = 0x10;
    let prepared_script: Vec<u8> = crate::vm::utils::alloc_bytearray::<96>(
        script_reg_id,
        points_bytearray.try_into().expect("qed"),
    )
    .into_iter()
    .collect();
    let script: Vec<u8> = vec![
        op::ecop(script_reg_id, RegId::ZERO, 0x01, script_reg_id),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect();

    ECOP_ALT_BN_128_METADATA.get_or_init(|| ZkInstructionMetadata {
        script_data: vec![],
        script: prepared_script
            .into_iter()
            .chain(script.into_iter())
            .collect(),
    })
}

static EPAR_ALT_BN_128_METADATA: OnceLock<ZkInstructionMetadata> = OnceLock::new();

fn epar_alt_bn_128_metadata() -> &'static ZkInstructionMetadata {
    let number_of_points = 18; // 1M gas limit
    let mut script_data = Vec::with_capacity(number_of_points as usize);
    for _ in 0u32..number_of_points {
        script_data.extend(
            hex::decode(
                "0000000000000000000000000000000000000000000000000000000000000001\
                0000000000000000000000000000000000000000000000000000000000000002\
                198e9393920d483a7260bfb731fb5d25f1aa493335a9e71297e485b7aef312c2\
                1800deef121f1e76426a00665e5c4479674322d4f75edadd46debd5cd992f6ed\
                090689d0585ff075ec9e99ad690c3395bc4b313370b38ef355acdadcd122975b\
                12c85ea5db8c6deb4aab71808dcb408fe3d1e7690c43d37b4ce6cc0166fa7daa",
            )
            .unwrap(),
        );
    }

    let script = vec![
        op::movi(0x11, number_of_points),
        op::gtf_args(0x10, 0x00, GTFArgs::ScriptData),
        op::epar(0x12, RegId::ZERO, 0x11, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
    .into_iter()
    .collect();

    EPAR_ALT_BN_128_METADATA.get_or_init(|| ZkInstructionMetadata {
        script_data,
        script,
    })
}

impl AsRepr for ZkInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            Self::ECOP_ALT_BN_128 => ecop_alt_bn_128_metadata().script.clone(),
            Self::EPAR_ALT_BN_128 => epar_alt_bn_128_metadata().script.clone(),
        }
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        Some(match &self {
            Self::ECOP_ALT_BN_128 => ecop_alt_bn_128_metadata().script_data.clone(),
            Self::EPAR_ALT_BN_128 => epar_alt_bn_128_metadata().script_data.clone(),
        })
    }
}
