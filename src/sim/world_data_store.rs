use crate::assets::shaders::ShaderKey;
use crate::assets::{Asset, AssetHandle};
use crate::config::{Config, DataStoreConfig};
use crate::sim::local_disk_data_store::LocalDiskDataStore;
use async_trait::*;

/// Database for all the assets that can be loaded into the current scene
#[async_trait]
pub trait WorldDataStore {}

pub async fn build_data_store(config: &Config) -> Box<impl WorldDataStore> {
    match config.data_store.clone() {
        DataStoreConfig::LocalDisk { root, entry_scene } => {
            Box::new(LocalDiskDataStore::new(&config).await)
        }
    }
}
