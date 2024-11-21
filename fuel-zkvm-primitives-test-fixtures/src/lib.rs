mod counter_contract;
mod testnet_block;
mod utils;

use crate::utils::{generate_input_at_block_height, start_node, Service};
pub use fuel_zkvm_primitives_utils::vm::Instruction;
use fuels::{accounts::Account, prelude::WalletUnlocked, types::BlockHeight};
use fuels_core::types::transaction_builders::{BuildableTransaction, ScriptTransactionBuilder};

async fn send_script_transaction(
    instruction: Instruction,
    wallet: &WalletUnlocked,
) -> anyhow::Result<BlockHeight> {
    let script = instruction.repr();

    let mut builder = ScriptTransactionBuilder::default().with_script(script);
    wallet.add_witnesses(&mut builder)?;
    wallet.adjust_for_fee(&mut builder, 0).await?;
    let provider = wallet.provider().expect("No provider");
    let tx = builder.build(provider).await?;

    let tx_id = provider.send_transaction(tx).await?;

    // Sleep to await the transaction inclusion in off chain database.
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;

    let inclusion_block_height = provider
        .get_transaction_by_id(&tx_id)
        .await
        .expect("No transaction")
        .expect("No transaction")
        .block_height
        .expect("No block height");

    Ok(inclusion_block_height)
}

/// We should move this to test-helpers once zkvm-perf doesn't have a dep on it
pub async fn start_node_with_transaction_and_produce_prover_input(
    instruction: Instruction,
) -> anyhow::Result<Service> {
    let (fuel_node, wallet) = start_node(None).await;
    let tx_inclusion_block_height = send_script_transaction(instruction, &wallet).await?;
    let service = generate_input_at_block_height(fuel_node, tx_inclusion_block_height).await?;
    Ok(service)
}

#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;

    async fn basic_alu_test(instruction: Instruction) {
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
                    basic_alu_test(Instruction::$instruction).await;
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
}
