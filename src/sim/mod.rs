mod local_disk_data_store;
pub mod world_data_store;

use crate::config::Config;
use legion::World;
use winit::event::WindowEvent;
use world_data_store::{build_data_store, WorldDataStore};

pub struct Simulation {
    data_store: Box<dyn WorldDataStore>,
    world: World,
}

impl Simulation {
    pub fn new(config: &Config) -> Self {
        Simulation {
            data_store: build_data_store(&config),
            world: World::default(),
        }
    }

    fn process_input(&mut self, event: &WindowEvent) -> bool {
        todo!()
    }

    fn update() {
        todo!()
    }
}
