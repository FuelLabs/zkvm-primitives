use fuel_core_executor::ports::RelayerPort;
use fuel_core_relayer::storage::EventsHistory;
use fuel_core_storage::{
    Error as StorageError,
    StorageAsRef,
    StorageInspect,
};
use fuel_core_types::{
    blockchain::primitives::DaBlockHeight,
    services::relayer::Event,
};
use std::{
    cell::RefCell,
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct RelayerRecorder<S> {
    storage: S,
    record: Arc<RefCell<prover::Relayer>>,
}

impl<S> RelayerRecorder<S> {
    pub fn new(storage: S) -> Self {
        Self {
            storage,
            record: Default::default(),
        }
    }

    pub fn into_prover_relayer(self) -> prover::Relayer {
        self.record.borrow().clone()
    }
}

impl<S> RelayerPort for RelayerRecorder<S>
where
    S: StorageInspect<EventsHistory, Error = StorageError>,
{
    fn enabled(&self) -> bool {
        true
    }

    fn get_events(&self, da_height: &DaBlockHeight) -> anyhow::Result<Vec<Event>> {
        let events = self
            .storage
            .storage_as_ref::<EventsHistory>()
            .get(da_height)?
            .map(|cow| cow.into_owned())
            .unwrap_or_default();

        self.record
            .borrow_mut()
            .add_event(*da_height, events.clone());

        Ok(events)
    }
}
