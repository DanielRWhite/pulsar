use crate::{ world::World, entity::Entity };
use std::sync::{ Arc, Mutex };

pub struct System { }

impl System {
        pub fn new() -> Self {
                System { }
        }

        pub fn get_name(&self) -> String {
                String::from("")
        }

        pub fn run_system(&self) { }

        pub fn get_entities(&self, _world: &World) -> Vec<Arc<Mutex<Entity>>> {
                Vec::new()
        }
}