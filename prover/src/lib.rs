#![deny(clippy::arithmetic_side_effects)]
#![deny(clippy::cast_possible_truncation)]
#![deny(unused_crate_dependencies)]
#![deny(warnings)]

use crate::memory::PanicStorage;
use alloc::collections::BTreeMap;
use alloy_sol_types::{
    private::U256,
    sol,
};
use core::cell::RefCell;
use fuel_core_executor::{
    executor::{
        ExecutionInstance,
        ExecutionOptions,
    },
    ports::RelayerPort,
};
use fuel_core_storage::transactional::{
    Changes,
    ConflictPolicy,
    StorageTransaction,
};
use fuel_core_types::{
    blockchain::{
        block::Block,
        primitives::DaBlockHeight,
    },
    fuel_crypto,
    services::{
        executor::{
            Error as ExecutorError,
            Result as ExecutorResult,
        },
        relayer::Event,
    },
};

extern crate alloc;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint256 input_hash;
        uint256 block_id;
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Input {
    pub block: Block,
    pub storage: Changes,
    pub relayer: Relayer,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Relayer(RefCell<BTreeMap<DaBlockHeight, Vec<Event>>>);

impl Relayer {
    pub fn new() -> Self {
        Self(RefCell::new(BTreeMap::new()))
    }

    pub fn add_event(&self, da_block_height: DaBlockHeight, events: Vec<Event>) {
        self.0.borrow_mut().insert(da_block_height, events);
    }
}

impl RelayerPort for Relayer {
    fn enabled(&self) -> bool {
        true
    }

    fn get_events(&self, da_block_height: &DaBlockHeight) -> anyhow::Result<Vec<Event>> {
        self.0
            .borrow_mut()
            .remove(da_block_height)
            .ok_or_else(|| anyhow::anyhow!("Not found"))
    }
}

mod memory {
    use fuel_core_storage::{
        column::Column,
        kv_store::{
            KeyValueInspect,
            Value,
        },
        Result as StorageResult,
    };

    #[derive(Default, Clone, Debug, PartialEq, Eq)]
    pub struct PanicStorage;

    impl KeyValueInspect for PanicStorage {
        type Column = Column;

        fn get(&self, _: &[u8], _: Self::Column) -> StorageResult<Option<Value>> {
            panic!(
                "The panic storage does not support reading. \
                All data should be provided via the `Input` struct.\
                If the execution hits this panic, it means that the prover \
                is not set up correctly. Or the execution is impossible."
            )
        }
    }
}

pub fn prove(input_bytes: &[u8]) -> ExecutorResult<PublicValuesStruct> {
    let input: Input = bincode::deserialize_from(input_bytes)
        .map_err(|e| ExecutorError::Other(format!("Unable to decode the input {e}")))?;

    let Input {
        block,
        storage,
        relayer,
    } = input;

    let panic_storage = PanicStorage::default();
    let storage =
        StorageTransaction::transaction(panic_storage, ConflictPolicy::Fail, storage);

    let validator = ExecutionInstance::new(
        relayer,
        storage,
        ExecutionOptions {
            extra_tx_checks: true,
            backtrace: false,
        },
    );

    // We don't need artifacts from validation
    let _ = validator.validate_without_commit(&block)?;

    // Prepare return values
    let input_hash = fuel_crypto::Hasher::hash(input_bytes);
    let block_id = block.header().id();

    let proof = PublicValuesStruct {
        input_hash: U256::from_be_bytes(input_hash.into()),
        block_id: U256::from_be_bytes(block_id.into()),
    };
    Ok(proof)
}
