use futures::executor::block_on;
use std::cell::Cell;
use stretch::assets::asset_ref::*;
use stretch::assets::AssetError;

pub struct MockLoadStrategy {
    mock_asset_data: Vec<u8>,
    override_error: Option<AssetError>,
    pub call_count: Cell<u32>,
}

impl MockLoadStrategy {
    pub fn new_with_data(data: Vec<u8>) -> MockLoadStrategy {
        MockLoadStrategy {
            mock_asset_data: data,
            override_error: None,
            call_count: Cell::new(0),
        }
    }
}

impl AssetLoadStrategy for MockLoadStrategy {
    async fn load_asset_data(&self) -> Result<Vec<u8>, AssetError> {
        self.call_count.replace(self.call_count + 1);

        if let Some(error) = self.override_error {
            return Err(error);
        }

        Ok(self.mock_asset_data)
    }
}

#[test]
pub fn asset_references_do_not_execute_the_load_strategy_until_requested() {
    let mock_data: Vec<u8> = vec![5, 3, 4];
    let asset_ref = AssetRef::new(Box::new(MockLoadStrategy::new_with_data(mock_data)));
    assert_eq!(
        asset_ref
            .load_strategy
            .dyn_into::<MockLoadStrategy>()
            .call_count,
        0
    );

    block_on(asset_ref.load_data());

    assert_eq!(
        asset_ref
            .load_strategy
            .dyn_into::<MockLoadStrategy>()
            .call_count,
        1
    );
}

#[test]
pub fn assets_properly_error_when_the_load_strategy_fails() {
    let asset_ref = AssetRef::new(Box::new(MockLoadStrategy::new_with_error(
        AssetError::PathNotFound(String::from("test error")),
    )));
    let result = block_on(asset_ref.load_data());
    assert!(result.is_err());
}
