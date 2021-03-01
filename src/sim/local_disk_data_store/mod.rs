use crate::assets::AssetHandle;
use crate::config::DataStoreConfig;
use super::WorldDataStore;
use async_trait::async_trait;
use gltf::*;
use slotmap::{SlotMap, SecondaryMap};
use walkdir::{DirEntry, WalkDir};
use std::collections::HashMap;
use crate::assets::asset_manifest::AssetManifest;

pub struct LocalDiskDataStore {
    root_dir: String,
}

pub struct LocalDiskLoadStrategy {
    file: String,
    file_type: String,
}

impl LocalDiskDataStore {
    pub async fn new(root_dir: &str, entry_scene: &str) -> Self {
        let mut manifest = AssetManifest::default();
        let file_iter = WalkDir::new(root_dir).into_iter()
            .filter_entry(|e| !e.file_name().to_str().starts_with("."))
            .filter(|e| e.is_ok())
            .map(|e|e.unwrap());

        for file in file_iter {
            let filename = match file.file_name().to_str() {
                Ok(f) => f,
                Err(err) =>
            }
            manifest.register_asset_at_path(file.file_name().to_str(), Box::new(

            ))
        }

        gltf::import()
        LocalDiskDataStore {
            root_dir: root_dir.to_string(),
        }
    }
}



#[async_trait]
impl WorldDataStore for LocalDiskDataStore {
}
