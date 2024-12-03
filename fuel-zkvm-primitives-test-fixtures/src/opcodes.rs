#![allow(unused)]

use crate::utils::{
    generate_input_at_block_height, get_temp_db, start_node, start_node_with_db, Service,
};
use fuel_core::database::balances::BalancesInitializer;
use fuel_core::database::database_description::on_chain::OnChain;
use fuel_core::database::state::StateInitializer;
use fuel_core::database::Database;
use fuel_core::service::{FuelService, SharedState};
use fuel_core_storage::rand::prelude::StdRng;
use fuel_core_storage::rand::{RngCore, SeedableRng};
use fuel_core_storage::tables::{Coins, ContractsLatestUtxo, ContractsRawCode};
use fuel_core_storage::vm_storage::IncreaseStorageKey;
use fuel_core_storage::{StorageAsMut, StorageAsRef};
use fuel_core_types::entities::coins::coin::{CompressedCoin, CompressedCoinV1};
use fuel_core_types::entities::contract::ContractUtxoInfo;
use fuel_core_types::fuel_crypto;
use fuel_core_types::fuel_tx::{AssetId, Bytes32, Executable};
use fuel_core_types::fuel_types::Address;
use fuel_zkvm_primitives_utils::vm::base::AsRepr;
use fuel_zkvm_primitives_utils::vm::blob::BlobInstruction;
use fuel_zkvm_primitives_utils::vm::contract::{ContractInstruction, ContractMetadata};
pub use fuel_zkvm_primitives_utils::vm::Instruction;
use fuels::prelude::Contract;
use fuels::{accounts::Account, prelude::WalletUnlocked, types::BlockHeight};
use fuels_core::types::coin::Coin;
use fuels_core::types::coin_type::CoinType;
use fuels_core::types::input::Input;
use fuels_core::types::output::Output;
use fuels_core::types::transaction_builders::{
    Blob, BlobTransactionBuilder, BuildableTransaction, ScriptTransactionBuilder,
    TransactionBuilder,
};
use fuels_core::types::tx_status::TxStatus;

async fn send_script_transaction(
    instruction: Instruction,
    wallet: &WalletUnlocked,
) -> anyhow::Result<BlockHeight> {
    let script = instruction.repr();
    let script_data = instruction.script_data();

    let additional_inputs = instruction.additional_inputs();
    let additional_outputs = instruction.additional_outputs();

    let mut builder = ScriptTransactionBuilder::default().with_script(script);

    if let Some(script_data) = script_data {
        builder = builder.with_script_data(script_data);
    }

    if let Some(additional_inputs) = additional_inputs {
        builder = builder.with_inputs(additional_inputs);
    }

    if let Some(additional_outputs) = additional_outputs {
        builder = builder.with_outputs(additional_outputs);
    }

    wallet.add_witnesses(&mut builder)?;
    wallet.adjust_for_fee(&mut builder, 0).await?;
    let provider = wallet.provider().expect("No provider");
    let tx = builder.build(provider).await?;

    let tx_id = provider.send_transaction(tx).await?;

    // Sleep to await the transaction inclusion in off chain database.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let tx = provider
        .get_transaction_by_id(&tx_id)
        .await
        .expect("No transaction")
        .expect("No transaction");

    let revert_reason = match tx.status {
        TxStatus::Success { .. } => {
            return Err(anyhow::anyhow!("Transaction should have reverted"))
        }
        TxStatus::Submitted => return Err(anyhow::anyhow!("Transaction should have executed")),
        TxStatus::SqueezedOut { reason } => {
            return Err(anyhow::anyhow!(
                "Transaction should have been included and reverted: {}",
                reason
            ))
        }
        TxStatus::Revert { reason, .. } => reason,
    };

    assert_eq!(revert_reason, "OutOfGas");

    let inclusion_block_height = tx.block_height.expect("No block height");

    Ok(inclusion_block_height)
}

async fn send_blob_transaction(
    instruction: BlobInstruction,
    wallet: &WalletUnlocked,
) -> anyhow::Result<BlockHeight> {
    let blob_data = instruction.blob_data();

    let blob = Blob::new(blob_data);

    let mut builder = BlobTransactionBuilder::default().with_blob(blob);
    wallet.adjust_for_fee(&mut builder, 0).await?;
    wallet.add_witnesses(&mut builder)?;

    let provider = wallet.provider().expect("No provider");

    let tx = builder.build(&provider).await?;
    provider
        .send_transaction_and_await_commit(tx)
        .await?
        .check(None)?;

    send_script_transaction(Instruction::BLOB(instruction), wallet).await
}

async fn scaffold_contract_instruction(
    db: &mut Database<OnChain>,
    instruction: ContractInstruction,
) -> anyhow::Result<()> {
    let contract_metadata = instruction.contract_metadata();

    if let Some(ContractMetadata {
        contract_id,
        contract_bytecode,
        state_size,
        predicate_metadata,
    }) = contract_metadata
    {
        db.storage_as_mut::<ContractsRawCode>()
            .insert(&contract_id, &contract_bytecode)?;

        // need this for existence checks
        db.storage_as_mut::<ContractsLatestUtxo>()
            .insert(&contract_id, &ContractUtxoInfo::default())?;

        // assets, storage
        let mut storage_key = primitive_types::U256::zero();
        let mut key_bytes = Bytes32::zeroed();

        db.init_contract_state(
            &contract_id,
            (0..state_size).map(|_| {
                storage_key.to_big_endian(key_bytes.as_mut());
                storage_key.increase().unwrap();
                (key_bytes, key_bytes.to_vec())
            }),
        )?;

        let mut storage_key = primitive_types::U256::zero();
        let mut sub_id = Bytes32::zeroed();
        db.init_contract_balances(
            &contract_id,
            (0..state_size as u64).map(|k| {
                storage_key.to_big_endian(sub_id.as_mut());

                let asset = if k % 2 == 0 {
                    let hasher = fuel_crypto::Hasher::default();
                    AssetId::new(
                        *hasher
                            .chain(contract_id.as_slice())
                            .chain(sub_id.as_slice())
                            .finalize(),
                    )
                } else {
                    let asset_id = AssetId::new(*sub_id);
                    storage_key.increase().unwrap();
                    asset_id
                };
                (asset, k / 2 + 1_000)
            }),
        )?;

        if let Some(predicate_metadata) = predicate_metadata {
            // insert into coins table
            db.storage_as_mut::<Coins>().insert(
                &predicate_metadata.predicate_utxo_id,
                &CompressedCoin::V1(CompressedCoinV1 {
                    owner: Address::from(predicate_metadata.predicate_owner),
                    amount: predicate_metadata.coin_amount,
                    asset_id: AssetId::zeroed(),
                    tx_pointer: Default::default(),
                }),
            )?;
        }
    }

    Ok(())
}

/// We should move this to test-helpers once zkvm-perf doesn't have a dep on it
pub async fn start_node_with_transaction_and_produce_prover_input(
    instruction: Instruction,
) -> anyhow::Result<Service> {
    let (fuel_node, wallet, tx_inclusion_block_height) = match instruction {
        Instruction::BLOB(instruction) => {
            let (fuel_node, wallet) = start_node(None).await;
            let block_height = send_blob_transaction(instruction, &wallet).await?;
            (fuel_node, wallet, block_height)
        }
        Instruction::CONTRACT(instruction) => {
            let mut db = get_temp_db();
            scaffold_contract_instruction(&mut db, instruction).await?;
            let (fuel_node, wallet) = start_node_with_db(db, None).await;
            let block_height =
                send_script_transaction(Instruction::CONTRACT(instruction), &wallet).await?;
            (fuel_node, wallet, block_height)
        }
        _ => {
            let (fuel_node, wallet) = start_node(None).await;
            let block_height = send_script_transaction(instruction, &wallet).await?;
            (fuel_node, wallet, block_height)
        }
    };

    generate_input_at_block_height(fuel_node, tx_inclusion_block_height).await
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use fuel_zkvm_primitives_utils::vm::alu::AluInstruction;
    use fuel_zkvm_primitives_utils::vm::blob::BlobInstruction;
    use fuel_zkvm_primitives_utils::vm::contract::ContractInstruction;
    use fuel_zkvm_primitives_utils::vm::control::ControlInstruction;
    use fuel_zkvm_primitives_utils::vm::crypto::CryptoInstruction;
    use fuel_zkvm_primitives_utils::vm::memory::MemoryInstruction;
    use fuel_zkvm_primitives_utils::vm::other::OtherInstruction;

    async fn basic_opcode_test(instruction: Instruction) {
        let service = start_node_with_transaction_and_produce_prover_input(instruction)
            .await
            .unwrap();
        let serialized_input = bincode::serialize(&service.input).unwrap();
        let proof = fuel_zkvm_primitives_prover::prove(&serialized_input).unwrap();
        let block_id: [u8; 32] = service.input.block.header().id().into();
        assert_eq!(block_id, proof.block_id.to_be_bytes())
    }

    macro_rules! alu_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_alu_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::ALU(AluInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! control_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_ctrl_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CTRL(ControlInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! memory_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_mem_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::MEM(MemoryInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! blob_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_blob_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::BLOB(BlobInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! crypto_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_crypto_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CRYPTO(CryptoInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! other_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_other_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::OTHER(OtherInstruction::$instruction)).await;
                }
            }
        };
    }

    macro_rules! contract_test {
        ($instruction:ident) => {
            paste::paste! {
                #[tokio::test]
                async fn [<test_contract_instruction_ $instruction:lower>]() {
                    basic_opcode_test(Instruction::CONTRACT(ContractInstruction::$instruction)).await;
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
    // contract_test!(TR); something up here
    contract_test!(SWW);
    contract_test!(SWWQ);
    contract_test!(SRW);
    contract_test!(SRWQ);
    contract_test!(SCWQ);
    contract_test!(SMO);
}
