use crate::assets::shaders::ShaderKey;
use crate::sim::world_data_store::WorldDataStore;
use async_trait::async_trait;

pub struct LocalDiskDataStore {
    root_dir: String,
}

impl LocalDiskDataStore {
    pub fn new(root_dir: &str) -> Self {
        LocalDiskDataStore {
            root_dir: root_dir.to_string(),
        }
    }
}

#[async_trait]
impl WorldDataStore for LocalDiskDataStore {
    async fn populate_manifest(&mut self) -> Result<(), String> {
        unimplemented!()
    }

    async fn load_shader(&self, uri: &str) -> Result<ShaderKey, String> {
        unimplemented!()
    }
}
