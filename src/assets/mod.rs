pub mod asset_manifest;
pub mod asset_ref;
pub mod materials;
pub mod shaders;

use slotmap::*;

/// Id type we can use to reference assets
new_key_type! { pub struct AssetHandle; }

pub enum AssetError {
    PathNotFound(String),
}
