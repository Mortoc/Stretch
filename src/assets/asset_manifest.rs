use crate::assets::asset_ref::{AssetLoadStrategy, AssetRef};
use crate::assets::AssetHandle;
use slotmap::SlotMap;
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetManifest {
    /// Get the asset handle from its path name
    paths: HashMap<String, AssetHandle>,
    manifest: SlotMap<AssetHandle, AssetReference>,
}

impl AssetManifest {
    pub fn register_asset_at_path(
        &mut self,
        path: &str,
        load_strategy: Box<dyn AssetLoadStrategy>,
    ) -> AssetHandle {
        let handle = self.manifest.insert(AssetRef::new(load_strategy));
        paths.insert(String::from(&path), handle);
        handle
    }
}
