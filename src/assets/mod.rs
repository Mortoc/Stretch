pub mod materials;
pub mod shaders;

/// Trait Asset provides tracking information per session and across sessions for assets. Implementors
/// are responsible for cleaning during Drop.
pub trait Asset {
    /// UUID for this asset unique to this session
    fn instance_id(&self) -> u64;

    /// Uniform Resource Identifier for this asset, deterministic across sessions. Typically based
    /// on the path the asset was loaded from.
    fn uri(&self) -> &str;
}
