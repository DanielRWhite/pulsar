use ahash::{HashMap, HashMapExt};

use crate::{
	entity::Entity,
	component::Component
};

use std::{
	sync::{ Arc, Mutex },
	io::{ Error, ErrorKind },
};

// Keys are unique lists of component types regardless of order
// Values are entities
pub struct Archetypes {
	table: Vec<String>,
	entities: HashMap<Vec<String>, Vec<Arc<Mutex<Box<Entity>>>>>
}

impl Archetypes {
	pub fn new() -> Self {
		let table = Vec::<String>::new();
		let entities = HashMap::<Vec<String>, Vec<Arc<Mutex<Box<Entity>>>>>::new();

		Archetypes { table, entities }
	}

	pub fn add_entity(&mut self, entity: Arc<Mutex<Box<Entity>>>, components: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
		if self.entities.contains_key(&components) {
			let archetype = match self.entities.get_mut(&components) {
				Some(res) => res,
				None => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to get mutable instance of archetypes")))
			};

			if archetype.iter().position(|e| Arc::ptr_eq(e, &entity)).is_none() {
				archetype.push(entity);
			}
			
			//archetype.push(Arc::new(Mutex::new(Box::new(entity))));

			// if archetype.iter().position(|e| {
			// 	let arc = Arc::clone(e);
			// 	let arc = match Arc::try_unwrap(arc) {
			// 		Ok(res) => res,
			// 		Err(_) => return false
			// 	};

			// 	match arc.lock() {
			// 		Ok(locked) => return **locked == entity,
			// 		Err(_) => return false
			// 	};

			// }).is_none() {
			// 	archetype.push(Arc::new(Mutex::new(Box::new(entity))));
			// }
		} else {
			self.entities.insert(components, vec![ entity ]);
		}

		Ok(())
	}

	pub fn remove_entity(&mut self, entity: Arc<Mutex<Box<Entity>>>, components: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
		if self.entities.contains_key(&components) {
			let archetype = match self.entities.get_mut(&components) {
				Some(res) => res,
				None => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to get mutable instance of archetypes")))
			};

			match archetype.iter().position(|e| Arc::ptr_eq(e, &entity)) {
				Some(pos) => { archetype.remove(pos); },
				None => { }
			}

			return Ok(())
		}

		return Err(Box::new(Error::new(ErrorKind::Other, "Entity archetype not found")))
	}

	pub fn get_entities(&self, archetype: Vec<String>) -> Option<Vec<Arc<Mutex<Box<Entity>>>>> {
		match self.entities.get(&archetype) {
			Some(entities) => Some(entities.clone()),
			None => None
		}
	}
}