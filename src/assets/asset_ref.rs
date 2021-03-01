use crate::assets::AssetError;

#[async_trait]
pub trait AssetLoadStrategy {
    async fn load_asset_data(&self) -> Result<Vec<u8>, AssetError>;
}

/// Reference to the raw data of an asset, which may or may not be loaded already.
pub struct AssetRef {
    /// Data is populated when we load the asset from its source
    data: Option<Vec<u8>>,
    pub load_strategy: Box<dyn AssetLoadStrategy>,
}

impl AssetRef {
    pub fn new(load_strategy: Box<dyn AssetLoadStrategy>) -> AssetRef {
        AssetRef {
            data: None,
            load_strategy,
        }
    }

    /// Returns the data immediately if it's loaded, otherwise it will load it asynchronously
    pub async fn load_data(&mut self) -> Result<Vec<u8>, AssetError> {
        if let Some(data) = self.data {
            return Ok(data);
        }
        self.load_strategy.load_asset_data().await
    }
}
