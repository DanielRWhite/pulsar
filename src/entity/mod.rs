use crate::{ world::{ World, entity_manager::keys::EntityArchetypeKey }, component::Component };
use ahash::{ HashMap, HashMapExt };
use std::{
        io::{ Error, ErrorKind },
        error::Error as ErrorTrait
};

pub struct Entity {
        id: usize,
        components: HashMap<String, Box<dyn Component>>
}

impl Entity {
        pub fn new(world: &mut World) -> Result<Self, Box<dyn ErrorTrait>> {
                let id = world.register_id()?;
                let components: HashMap<String, Box<dyn Component>> = HashMap::new();

                Ok(Entity { id, components })
        }

        pub fn attach_component(&mut self, component: Box<dyn Component>) -> Result<(), Box<dyn ErrorTrait>> {
                let archetype = component.get_archetype();
                if self.components.contains_key(&archetype) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity already contains component of that type")))
                }

                self.components.insert(archetype, component);

                Ok(())
        }

        pub fn remove_component(&mut self, component: String) {
                self.components.remove(&component);
        }

        pub fn get_id(&self) -> usize {
                self.id.clone()
        }

        pub fn get_archetype(&self) -> EntityArchetypeKey {
                EntityArchetypeKey::from_strings(self.components.iter().map(|(_, x)| x.get_archetype()).collect::<Vec<String>>())
        }
}