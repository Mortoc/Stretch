use crate::sim::Simulation;
use cgmath::*;
use winit::error::ExternalError;

pub mod web_renderer;
use crate::config::{Config, RendererConfig};
use futures::executor::block_on;
use web_renderer::WebRenderer;
use winit::window::Window;

pub trait Renderer {
    fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) -> Result<(), ExternalError>;
    fn render(&self, scene: &Simulation) -> Result<(), ExternalError>;
}

pub fn build_renderer(config: &Config, window: &Window) -> Box<dyn Renderer> {
    match config.renderer {
        RendererConfig::WebGPU => Box::new(block_on(WebRenderer::new(&window))),
        _ => unimplemented!("Unexpected Renderer"),
    }
}
