pub use fuel_zkvm_primitives_utils::vm::Instruction;
use include_dir::{include_dir, Dir};

static OPCODES_FIXTURES: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/fixtures/opcodes");

pub fn get_opcode_input(instruction: Instruction) -> Vec<u8> {
    let file = OPCODES_FIXTURES
        .get_file(format!("{instruction:?}.bin"))
        .unwrap();
    let serialized_input = file.contents().to_vec();

    serialized_input
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use fuel_zkvm_primitives_prover::Input;
    use fuel_zkvm_primitives_utils::vm::alu::AluInstruction;
    use fuel_zkvm_primitives_utils::vm::blob::BlobInstruction;
    use fuel_zkvm_primitives_utils::vm::contract::ContractInstruction;
    use fuel_zkvm_primitives_utils::vm::control::ControlInstruction;
    use fuel_zkvm_primitives_utils::vm::crypto::CryptoInstruction;
    use fuel_zkvm_primitives_utils::vm::memory::MemoryInstruction;
    use fuel_zkvm_primitives_utils::vm::other::OtherInstruction;

    fn basic_opcode_test(instruction: Instruction) {
        let serialized_input = get_opcode_input(instruction);
        let deserialized_input = bincode::deserialize::<Input>(&serialized_input).unwrap();
        let proof = fuel_zkvm_primitives_prover::prove(&serialized_input).unwrap();
        let block_id: [u8; 32] = deserialized_input.block.header().id().into();
        assert_eq!(block_id, proof.block_id.to_be_bytes())
    }

    macro_rules! alu_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_alu_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::ALU(AluInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! control_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_ctrl_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CTRL(ControlInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! memory_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_mem_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::MEM(MemoryInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! blob_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_blob_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::BLOB(BlobInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! crypto_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_crypto_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CRYPTO(CryptoInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! other_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_other_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::OTHER(OtherInstruction::$instruction));
                }
            }
        };
    }

    macro_rules! contract_test {
        ($instruction:ident) => {
            paste::paste! {
                #[test]
                fn [<test_contract_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CONTRACT(ContractInstruction::$instruction));
                }
            }
        };
    }

    // ALU Tests. Compare the number with alu.rs
    // TODO: maybe proc-macro's can simplify this
    alu_test!(ADD);
    alu_test!(ADDI);
    alu_test!(AND);
    alu_test!(ANDI);
    alu_test!(DIV);
    alu_test!(DIVI);
    alu_test!(EQ);
    alu_test!(EXP);
    alu_test!(EXPI);
    alu_test!(GT);
    alu_test!(LT);
    alu_test!(MLOG);
    alu_test!(MOD);
    alu_test!(MODI);
    alu_test!(MOVE);
    alu_test!(MOVI);
    alu_test!(MROO);
    alu_test!(MUL);
    alu_test!(MULI);
    alu_test!(MLDV);
    alu_test!(NOOP);
    alu_test!(NOT);
    alu_test!(OR);
    alu_test!(ORI);
    alu_test!(SLL);
    alu_test!(SLLI);
    alu_test!(SRL);
    alu_test!(SRLI);
    alu_test!(SUB);
    alu_test!(SUBI);
    alu_test!(WDCM);
    alu_test!(WDOP);
    alu_test!(WDML);
    alu_test!(WDDV);
    alu_test!(WDMD);
    alu_test!(WDAM);
    alu_test!(WDMM);
    alu_test!(WQCM);
    alu_test!(WQOP);
    alu_test!(WQML);
    alu_test!(WQDV);
    alu_test!(WQMD);
    alu_test!(WQAM);
    alu_test!(WQMM);
    alu_test!(XOR);
    alu_test!(XORI);

    // Control Tests. Compare the number with control.rs
    control_test!(JMP);
    control_test!(JMPB);
    control_test!(JMPF);
    control_test!(JI);
    control_test!(JNE);
    control_test!(JNEB);
    control_test!(JNEF);
    control_test!(JNEI);
    control_test!(JNZB);
    control_test!(JNZF);
    control_test!(JNZI);

    // Memory Tests. Compare the number with memory.rs
    memory_test!(ALOC);
    memory_test!(CFE);
    memory_test!(CFEI);
    memory_test!(CFS);
    memory_test!(CFSI);
    memory_test!(LB);
    memory_test!(LW);
    memory_test!(MCL);
    memory_test!(MCLI);
    memory_test!(MCP);
    memory_test!(MCPI);
    memory_test!(MEQ);
    memory_test!(POPH);
    memory_test!(POPL);
    memory_test!(PSHH);
    memory_test!(PSHL);
    memory_test!(SB);
    memory_test!(SW);

    // Blob Tests. Compare the number with blob.rs
    blob_test!(BSIZ);
    blob_test!(BLDD);

    // Crypto Tests. Compare the number with crypto.rs
    crypto_test!(ECK1);
    crypto_test!(ECR1);
    crypto_test!(ED19);
    crypto_test!(K256);
    crypto_test!(S256);

    // Other Tests. Compare the number with other.rs
    other_test!(GM);
    other_test!(GTF);
    other_test!(FLAG);

    // Contract Tests. Compare the number with contract.rs
    contract_test!(BHEI);
    contract_test!(BHSH);
    contract_test!(CB);
    contract_test!(LOG);
    contract_test!(TIME);
    contract_test!(BAL);
    contract_test!(BURN);
    contract_test!(CCP);
    contract_test!(CROO);
    contract_test!(CSIZ);
    contract_test!(LDC);
    contract_test!(LOGD);
    contract_test!(MINT);
    contract_test!(RETD);
    contract_test!(TR);
    contract_test!(SWW);
    contract_test!(SWWQ);
    contract_test!(SRW);
    contract_test!(SRWQ);
    contract_test!(SCWQ);
    contract_test!(SMO);
    contract_test!(CALL);

    // special test cases
    // this was generated with 30M gas limit by modifying test_consensus_parameters.json
    contract_test!(SPECIAL_MEMORY_STACK_AND_HEAP_ALLOCS);
}
