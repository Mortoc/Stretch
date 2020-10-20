use serde::{Deserialize, Serialize};

/// HDR-Compatible Color that can be submitted to the GPU
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Color {
    rgba: [f32; 4],
}

impl Color {
    /// Create a color from separate RGB and A values
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { rgba: [r, g, b, a] }
    }

    /// Shortcut to build a pure red value
    pub fn red() -> Color {
        Color {
            rgba: [1.0, 0.0, 0.0, 1.0],
        }
    }

    /// Shortcut to build a pure green value
    pub fn green() -> Color {
        Color {
            rgba: [0.0, 1.0, 0.0, 1.0],
        }
    }

    /// Shortcut to build a pure blue value
    pub fn blue() -> Color {
        Color {
            rgba: [0.0, 0.0, 1.0, 1.0],
        }
    }

    /// Get the Red component of this color
    pub fn r(&self) -> f32 {
        self.rgba[0]
    }

    /// Get the Green component of this color
    pub fn g(&self) -> f32 {
        self.rgba[1]
    }

    /// Get the Blue component of this color
    pub fn b(&self) -> f32 {
        self.rgba[2]
    }

    /// Get the Alpha component of this color
    pub fn a(&self) -> f32 {
        self.rgba[3]
    }
}
