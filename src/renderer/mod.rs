use crate::sim::Simulation;
use cgmath::*;
use winit::error::ExternalError;

pub mod color;
pub mod web_renderer;

use crate::config::{Config, RendererConfig};
use futures::executor::block_on;
use web_renderer::WebRenderer;
use winit::window::Window;

/// The Renderer is the provider for the selected graphics backend (WebGPU, Metal, etc)
pub trait Renderer {
    fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>);
    fn render(&self, scene: &Simulation);
}

pub fn build_renderer(config: &Config, window: &Window) -> Box<dyn Renderer> {
    match config.renderer {
        RendererConfig::WebGPU => Box::new(block_on(WebRenderer::new(&window))),
        _ => unimplemented!("Unexpected Renderer"),
    }
}
