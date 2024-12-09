use crate::utils::{
    generate_input_at_block_height, get_temp_db, start_node, start_node_with_db, Service,
};
use fuel_core::database::balances::BalancesInitializer;
use fuel_core::database::database_description::on_chain::OnChain;
use fuel_core::database::state::StateInitializer;
use fuel_core::database::Database;
use fuel_core_storage::tables::{ContractsLatestUtxo, ContractsRawCode};
use fuel_core_storage::vm_storage::IncreaseStorageKey;
use fuel_core_storage::StorageAsMut;
use fuel_core_types::entities::contract::ContractUtxoInfo;
use fuel_core_types::fuel_crypto;
use fuel_core_types::fuel_tx::{AssetId, Bytes32};
use fuel_zkvm_primitives_utils::vm::base::AsRepr;
use fuel_zkvm_primitives_utils::vm::blob::BlobInstruction;
use fuel_zkvm_primitives_utils::vm::contract::{ContractInstruction, ContractMetadata};
use fuel_zkvm_primitives_utils::vm::{all_instructions, Instruction};
use fuels::{accounts::Account, prelude::WalletUnlocked, types::BlockHeight};
use fuels_core::types::transaction::Transaction;
use fuels_core::types::transaction_builders::{
    Blob, BlobTransactionBuilder, BuildableTransaction, ScriptTransactionBuilder,
    TransactionBuilder,
};
use fuels_core::types::tx_status::TxStatus;
use futures::stream::{self, StreamExt};

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

    let tx_id = tx.id(provider.chain_id());
    let tx_status = provider.send_transaction_and_await_commit(tx).await?;

    let revert_reason = match tx_status {
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

    let tx = provider
        .get_transaction_by_id(&tx_id)
        .await
        .expect("no tx")
        .expect("no tx");

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
    }

    Ok(())
}

pub async fn start_node_with_transaction_and_produce_prover_input(
    instruction: Instruction,
) -> anyhow::Result<Service> {
    let (fuel_node, tx_inclusion_block_height) = match instruction {
        Instruction::BLOB(instruction) => {
            let (fuel_node, wallet) = start_node(None).await;
            let block_height = send_blob_transaction(instruction, &wallet).await?;
            (fuel_node, block_height)
        }
        Instruction::CONTRACT(instruction) => {
            let mut db = get_temp_db();
            scaffold_contract_instruction(&mut db, instruction).await?;
            let (fuel_node, wallet) = start_node_with_db(db, None).await;
            let block_height =
                send_script_transaction(Instruction::CONTRACT(instruction), &wallet).await?;
            (fuel_node, block_height)
        }
        _ => {
            let (fuel_node, wallet) = start_node(None).await;
            let block_height = send_script_transaction(instruction, &wallet).await?;
            (fuel_node, block_height)
        }
    };

    generate_input_at_block_height(fuel_node, tx_inclusion_block_height).await
}

pub async fn generate_fixture() -> anyhow::Result<()> {
    let all_instructions = all_instructions();

    let concurrency_level = 8;

    stream::iter(all_instructions)
        .map(|instruction| async move {
            let service =
                start_node_with_transaction_and_produce_prover_input(instruction.clone()).await?;

            let serialized_prover_input = bincode::serialize(&service.input)?;
            let file_path = format!("src/fixtures/opcodes/{instruction:?}.bin");
            std::fs::write(file_path, serialized_prover_input)?;

            Ok::<(), anyhow::Error>(())
        })
        .buffer_unordered(concurrency_level)
        .for_each(|result| async {
            if let Err(e) = result {
                eprintln!("Error processing instruction: {:?}", e);
            }
        })
        .await;

    Ok(())
}
