//! The shaders module is responsible to representing `Shader` and `ShaderProgram` assets.
//!
//! A `ShaderProgram` is a compiled GPU program for a single stage of the graphics
//! pipeline (vertex, fragment, compute, etc)
//!
//! A `Shader` is a complete set of `ShaderPrograms` that represent a render pipeline (typically at
//! least consisting of a vertex and a fragment stage).

use crate::assets::{Asset, AssetHandle};
use crate::renderer::color::*;
use cgmath::{Vector2, Vector3, Vector4};
use serde::{Deserialize, Serialize};
use slotmap::*;

new_key_type! { pub struct ShaderKey; }
new_key_type! { pub struct ShaderProgramKey; }

pub struct Shader {
    display_name: String,
    uri: AssetHandle,
    instance_id: u64,
}

impl Asset for Shader {
    fn instance_id(&self) -> u64 {
        self.instance_id
    }

    fn uri(&self) -> AssetHandle {
        self.uri
    }
}

/// Named values that can be bound to a shader program
#[derive(Deserialize, Serialize)]
pub enum ShaderUniform {
    F1 { name: String, value: f32 },
    F2 { name: String, value: Vector2<f32> },
    F3 { name: String, value: Vector3<f32> },
    F4 { name: String, value: Vector4<f32> },
    Color { name: String, value: Color },
}
