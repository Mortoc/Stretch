use crate::renderer::color::*;
use cgmath::{Vector2, Vector3, Vector4};
use serde::{Deserialize, Serialize};
use slotmap::*;

new_key_type! { pub struct ShaderKey; }

pub struct Shader {
    display_name: String,
    uri: String,
    instance_id: u64,
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
