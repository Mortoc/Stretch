use super::shaders::ShaderKey;
use crate::assets::shaders::ShaderUniform;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct Material {
    display_name: String,
    uri: String,
    instance_id: u64,
    uniforms: Vec<ShaderUniform>,
    shader: Arc<ShaderKey>,
}

/// Configuration used to construct a Material Instance
#[derive(Deserialize, Serialize)]
pub struct MaterialConfig {
    /// Human-readable name for this type of material (ex: Unlit, PBR, etc)
    pub display_name: String,

    /// List of bind-able uniforms to attach resources to the inner shader program
    pub uniforms: Vec<ShaderUniform>,

    /// URI for the vertex program to use
    pub vert: String,

    /// URI for the fragment program to use
    pub frag: String,
}
