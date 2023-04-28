#[cfg(test)]
mod tests;

use crate::entity::EntityManager;

pub struct World {
        name: String,
        entities: EntityManager,
}

impl World {
        pub fn new(world_name: &str) -> World {
                let name: String = world_name.to_string();
                let entities: EntityManager = EntityManager::new();

                World { name, entities }
        }
}