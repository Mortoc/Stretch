use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub renderer: RendererConfig,
    pub data_store: DataStoreConfig,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "type")]
pub enum RendererConfig {
    WebGPU,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum DataStoreConfig {
    LocalDisk { root: String },
}
