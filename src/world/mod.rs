mod id_pool;

use std::sync::{ Arc, Mutex };
use id_pool::IDPool;
use crate::{
	entity::{ EntityBuilder, Entity },
	component::Component,
	archetypes::Archetypes
};

pub struct World {
	ids: IDPool,
	archetypes: Archetypes
}

impl World {
	pub fn new() -> Self {
		World {
			ids: IDPool::new(),
			archetypes: Archetypes::new()
		}
	}

	pub fn reserve_entity_id(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
		self.ids.reserve_entity_id()
	}

	pub fn dealloc_id(&mut self, id: usize) {
		self.ids.dealloc_id(id)
	}

	pub fn housekeeping(&mut self) {
		self.ids.housekeeping()
	}

	pub fn create_entity(&self) -> EntityBuilder {
		EntityBuilder::new()
	}

	pub fn add_entity(&mut self, entity: Entity) -> Result<(), Box<dyn std::error::Error>> {
		let components = entity.get_archetype();
		self.archetypes.add_entity(Arc::new(Mutex::new(Box::new(entity))), components)
	}

	pub fn remove_entity(&mut self, entity: Entity) -> Result<(), Box<dyn std::error::Error>> {
		let id = entity.get_id();
		let components = entity.get_archetype();

		self.archetypes.remove_entity(Arc::new(Mutex::new(Box::new(entity))), components)?;
		self.ids.dealloc_id(id);

		Ok(())
	}
}