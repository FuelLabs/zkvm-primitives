use fuel_core_storage::{
    kv_store::{
        KeyValueInspect,
        StorageColumn,
        Value,
        WriteOperation,
    },
    transactional::Changes,
    Result as StorageResult,
};
use std::{
    cell::RefCell,
    sync::Arc,
};

/// The in-memory storage for testing purposes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageAccessRecorder<S>
where
    S: KeyValueInspect,
{
    pub storage: S,
    record: Arc<RefCell<Changes>>,
}

impl<S> StorageAccessRecorder<S>
where
    S: KeyValueInspect,
{
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            record: Default::default(),
        }
    }

    pub fn into_changes(self) -> Changes {
        self.record.borrow().clone()
    }
}

impl<S> KeyValueInspect for StorageAccessRecorder<S>
where
    S: KeyValueInspect,
{
    type Column = S::Column;

    fn get(&self, key: &[u8], column: Self::Column) -> StorageResult<Option<Value>> {
        let value = self.storage.get(key, column)?;

        let mut record = self.record.borrow_mut();
        let tree = record.entry(column.id()).or_default();

        let key = key.to_vec().into();
        match &value {
            Some(value) => {
                tree.insert(key, WriteOperation::Insert(value.clone()));
            }
            None => {
                tree.insert(key, WriteOperation::Remove);
            }
        }
        Ok(value)
    }
}
