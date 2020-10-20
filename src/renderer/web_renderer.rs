use crate::renderer::Renderer;
use crate::sim::Simulation;
use cgmath::Vector2;
use wgpu::*;
use winit::error::ExternalError;
use winit::window::Window;

pub struct WebRenderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    swap_chain_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    size: winit::dpi::PhysicalSize<u32>,
}

impl Renderer for WebRenderer {
    fn resize(&mut self, new_size: &winit::dpi::PhysicalSize<u32>) -> Result<(), ExternalError> {
        self.size = *new_size;
        self.swap_chain_desc.width = new_size.width;
        self.swap_chain_desc.height = new_size.height;
        self.swap_chain = self
            .device
            .create_swap_chain(&self.surface, &self.swap_chain_desc);

        Ok(())
    }

    fn render(&self, scene: &Simulation) -> Result<(), ExternalError> {
        unimplemented!()
    }
}

impl WebRenderer {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = Instance::new(BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    features: Features::empty(),
                    limits: Limits::default(),
                    shader_validation: true,
                },
                None,
            )
            .await
            .unwrap();

        let swap_chain_desc = SwapChainDescriptor {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &swap_chain_desc);

        Self {
            surface,
            device,
            queue,
            swap_chain_desc,
            swap_chain,
            size,
        }
    }
}
