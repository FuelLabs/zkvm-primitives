use crate::vm::base::AsRepr;
use crate::vm::memory::set_full_word;
use fuel_core_storage::rand::prelude::StdRng;
use fuel_core_storage::rand::{RngCore, SeedableRng};
use fuel_core_types::fuel_asm::{op, GTFArgs, Instruction, RegId};
use fuels_core::types::transaction_builders::Blob;
use std::sync::OnceLock;

static BLOB_INSTANCE: OnceLock<Blob> = OnceLock::new();
const BLOB_SIZE: usize = 1_000;

// Global function to access the Blob
fn get_blob_instance() -> &'static Blob {
    BLOB_INSTANCE.get_or_init(|| {
        let rng = &mut StdRng::seed_from_u64(2322u64);
        let mut code = vec![0u8; BLOB_SIZE];
        rng.fill_bytes(&mut code);
        Blob::new(code)
    })
}

#[derive(Debug)]
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
}

impl BlobInstruction {
    pub fn scaffold(&self) -> Blob {
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
