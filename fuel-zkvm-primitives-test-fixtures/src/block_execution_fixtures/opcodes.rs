//! Test fixtures for all the opcodes

// we want to re-export all internal instructions too
pub use fuel_zkvm_primitives_utils::vm::{
    alu::AluInstruction, blob::BlobInstruction, contract::ContractInstruction,
    control::ControlInstruction, crypto::CryptoInstruction, memory::MemoryInstruction,
    other::OtherInstruction, zk::ZkInstruction, Instruction,
};

use include_dir::{include_dir, Dir};

static OPCODES_FIXTURES: Dir =
    include_dir!("$CARGO_MANIFEST_DIR/src/block_execution_fixtures/fixtures/opcodes");

/// Get prover input for a given opcode
pub fn get_opcode_input(instruction: Instruction) -> Vec<u8> {
    let file = OPCODES_FIXTURES
        .get_file(format!("{instruction:?}.bin"))
        .unwrap();
    let serialized_input = file.contents().to_vec();

    serialized_input
}
