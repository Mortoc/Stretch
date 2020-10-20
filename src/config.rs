use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub renderer: RendererConfig,
    pub data_store: DataStoreConfig,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum RendererConfig {
    WebGPU,
}

impl Default for RendererConfig {
    fn default() -> Self {
        RendererConfig::WebGPU
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DataStoreConfig {
    LocalDisk { root: String },
}

impl Default for DataStoreConfig {
    fn default() -> Self {
        DataStoreConfig::LocalDisk {
            root: "".to_string(),
        }
    }
}
