use crate::vm::base::AsRepr;
use crate::vm::memory::set_full_word;
use fuel_core_storage::rand::prelude::StdRng;
use fuel_core_storage::rand::{RngCore, SeedableRng};
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuels_core::types::transaction_builders::Blob;
use std::sync::OnceLock;

static BLOB_INSTANCE: OnceLock<Vec<u8>> = OnceLock::new();
const BLOB_SIZE: usize = 1_000;

// Global function to access the Blob
fn get_blob_instance() -> &'static Vec<u8> {
    BLOB_INSTANCE.get_or_init(|| {
        let rng = &mut StdRng::seed_from_u64(2322u64);
        let mut blob_data = vec![0u8; BLOB_SIZE];
        rng.fill_bytes(&mut blob_data);
        blob_data
    })
}

#[cfg_attr(
    feature = "enhanced_enums",
    derive(clap::ValueEnum, enum_iterator::Sequence)
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Copy, Clone)]
pub enum BlobInstruction {
    BSIZ,
    BLDD,
}

impl AsRepr for BlobInstruction {
    fn repr(&self) -> Vec<u8> {
        match &self {
            BlobInstruction::BSIZ => bsiz(),
            BlobInstruction::BLDD => bldd(),
        }
        .into_iter()
        .collect()
    }

    fn script_data(&self) -> Option<Vec<u8>> {
        let blob_data = get_blob_instance().clone();
        let blob = Blob::new(blob_data);
        Some(blob.id().to_vec())
    }
}

impl BlobInstruction {
    pub fn blob_data(&self) -> Vec<u8> {
        get_blob_instance().clone()
    }
}

fn bsiz() -> Vec<Instruction> {
    vec![
        op::gtf_args(0x10, RegId::ZERO, GTFArgs::ScriptData),
        op::bsiz(0x11, 0x10),
        op::jmpb(RegId::ZERO, 0),
    ]
}

fn bldd() -> Vec<Instruction> {
    let mut prepared = set_full_word(0x12, BLOB_SIZE.try_into().unwrap());
    prepared.extend(vec![
        op::gtf_args(0x11, RegId::ZERO, GTFArgs::ScriptData),
        op::aloc(0x12),
        op::bldd(RegId::HP, 0x11, RegId::ZERO, 0x12),
        op::jmpb(RegId::ZERO, 0),
    ]);

    prepared
}
