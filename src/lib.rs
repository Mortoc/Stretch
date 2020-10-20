#![warn(missing_docs)]

use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowBuilder},
};

use config::Config;
use renderer::*;
use sim::Simulation;

pub mod assets;
pub mod config;
pub mod renderer;
pub mod sim;
pub mod utility;

pub fn start_main_loop(config: &Config) -> (Window, EventLoopProxy<Event<'static, ()>>) {
    env_logger::init();
    let event_loop = EventLoop::<Event<()>>::with_user_event();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut renderer = build_renderer(&config, &window);
    let mut sim = Simulation::new(&config);

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::Resized(new_size) => {
                renderer.resize(new_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                renderer.resize(*new_inner_size);
            }
            WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
            WindowEvent::KeyboardInput { input, .. } => match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = ControlFlow::Exit,
                _ => {}
            },
            _ => {}
        },
        _ => {}
    });
}
