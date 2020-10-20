use crate::assets::shaders::ShaderKey;
use crate::assets::Asset;
use crate::config::{Config, DataStoreConfig};
use crate::sim::local_disk_data_store::LocalDiskDataStore;
use async_trait::*;

/// Database for all the assets that can be loaded into the current scene
#[async_trait]
pub trait WorldDataStore {
    /// Loads information about what assets are currently available in this datastore.
    /// Can be called again at any time to refresh the manifest.
    async fn populate_manifest(&mut self) -> Result<(), String>;

    async fn load_shader(&self, uri: &str) -> Result<ShaderKey, String>;
}

pub fn build_data_store(config: &Config) -> Box<impl WorldDataStore> {
    match config.data_store.clone() {
        DataStoreConfig::LocalDisk { root } => Box::new(LocalDiskDataStore::new(&root)),
    }
}
