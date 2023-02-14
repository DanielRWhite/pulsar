pub mod entity_manager;

use entity_manager::{ EntityManager, keys::EntityArchetypeKey };
use crate::{ entity::Entity, system::System };
use std::{
        sync::{ Arc, Mutex },
        error::Error as ErrorTrait
};

pub struct World {
        entities: EntityManager
}

impl World {
        pub fn new() -> Self {
                World {
                        entities: EntityManager::new()
                }
        }

        pub fn create_entity(&mut self, entity: Entity) -> Result<(), Box<dyn ErrorTrait>> {
                self.entities.create_entity(entity)
        }

        pub fn delete_entity(&mut self, id: usize, archetype: Option<EntityArchetypeKey>) -> Result<(), Box<dyn ErrorTrait>> {
                self.entities.delete_entity(id, archetype)
        }

        pub fn get_entity(&self, id: usize) -> Result<Arc<Mutex<Entity>>, Box<dyn ErrorTrait>> {
                self.entities.get_entity(id)
        }

        pub fn query(&self, archetype: EntityArchetypeKey) -> Vec<Arc<Mutex<Entity>>> {
                self.entities.query(archetype)
        }

        #[allow(dead_code)]
        pub fn register_system(&self) {
                todo!()
        }

        #[allow(dead_code)]
        pub fn unregister_system(&self) {
                todo!()
        }

        #[allow(dead_code)]
        pub fn update_system(&self, _system: impl System) {
                todo!()
        }

        #[allow(dead_code)]
        pub fn run_system(&self, _name: String) {
                todo!()
        }

        pub fn register_id(&mut self) -> Result<usize, Box<dyn ErrorTrait>> {
                self.entities.register_id()
        }

        pub fn deregister_id(&mut self, id: usize) -> Result<(), Box<dyn ErrorTrait>> {
                self.entities.deregister_id(id)
        }
}