use crate::vm::base::AsRepr;
use fuel_core_storage::rand::prelude::StdRng;
use fuel_core_storage::rand::{RngCore, SeedableRng};
use fuel_core_types::fuel_asm::{op, Instruction};
use fuels_core::types::transaction_builders::Blob;
use std::sync::OnceLock;

static BLOB_INSTANCE: OnceLock<Blob> = OnceLock::new();

// Global function to access the Blob
pub fn get_blob_instance() -> &'static Blob {
    BLOB_INSTANCE.get_or_init(|| {
        let rng = &mut StdRng::seed_from_u64(2322u64);
        let mut code = vec![0u8; 100000];
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
    //BSIZ: Blob size
    // Description	Set $rA to the size of the blob with ID equal to the 32 bytes in memory starting at $rB.
    // Operation	$rA = len(blob(MEM[$rB, 32]));
    // Syntax	bsiz $rA, $rB
    // Encoding	0x00 rA rB - -
    // Notes
    // Panic if:
    //
    // $rA is a reserved register
    // $rB + 32 overflows or > VM_MAX_RAM
    // Blob ID MEM[$rB, 32] is not found
    // we have get_blob_instance with which we can get the id to load into memory
    let blob = get_blob_instance();
    let id = blob.id();
    // first, load the blob id into memory
    // then copy the memory lcoation to the register
    // then get the length of the blob
    vec![
        op::movi(0x10, 32),
        op::aloc(0x10), // now 32 bytes is allocated
    ]
}

fn bldd() -> Vec<Instruction> {
    vec![]
}
