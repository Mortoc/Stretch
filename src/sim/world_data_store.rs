use crate::config::{Config, DataStoreConfig};
use crate::sim::local_disk_data_store::LocalDiskDataStore;

/// Container for all the assets that can be loaded into the current scene
pub trait WorldDataStore {}

pub fn build_data_store(config: &Config) -> Box<dyn WorldDataStore> {
    match config.data_store.clone() {
        DataStoreConfig::LocalDisk { root } => Box::new(LocalDiskDataStore::new(&root)),
    }
}
