use crate::sim::world_data_store::WorldDataStore;

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

impl WorldDataStore for LocalDiskDataStore {}
