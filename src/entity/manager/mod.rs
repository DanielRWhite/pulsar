pub mod keys;

use super::Entity;
use keys::EntityArchetypeKey;

use crate::component::Component;
use ahash::{ HashMap, HashMapExt };
use std::{
	error::Error as ErrorTrait,
	io::{ Error, ErrorKind },
	sync::{ Arc, Mutex }
};

pub struct IDPool {
	current_id: usize,
	used_ids: Vec<usize>,
	available_ids: Vec<usize>
}

impl IDPool {
	pub fn new() -> Self {
		let current_id: usize = usize::MIN;
		let used_ids: Vec<usize> = Vec::new();
		let available_ids: Vec<usize> = Vec::new();

		IDPool { current_id, used_ids, available_ids }
	}

	pub fn allocate_id(&mut self) -> Result<usize, Box<dyn ErrorTrait>> {
		if self.available_ids.len() > 0 {
			// Safe unwrap, since we know there are items in the vec
			return Ok(self.available_ids.pop().unwrap())
		}

		if self.current_id == usize::MAX {
			return Err(Box::new(Error::new(ErrorKind::Other, "Maximum amount of entity IDs reached")))
		}

		self.current_id = self.current_id + 1;
		self.used_ids.push(self.current_id.clone());

		Ok(self.current_id.clone())
	}

	pub fn deallocate_id(&mut self, id: usize) {
		if let Some(pos) = self.used_ids.iter().position(|x| x == &id) {
			self.used_ids.remove(pos);
			self.available_ids.push(id);
		}
	}

	pub fn housekeeping(&mut self) {
		todo!()
	}
}

pub struct EntityManager {
	id_pool: IDPool,
	// Entity is stored on the Heap, and HashMap stores Arc<_> Smart Pointer (8 Bytes per Smart Pointer)
	// Named Entities are optional, and their ID is the Vec<String> joined with "/" like a path, with the
	// final path being ":" instead of "/" and their entity ID. The hashmap only stores the namespace path
	// without the ":<id>" portion.
	// Examples: enemies/zombie:128, minion/normal/ranged:86400, etc.
	named_entities: HashMap<Vec<String>, Vec<Arc<Mutex<Entity>>>>,

	// Entity is stored on the Heap, and HashMap stores Arc<_> Smart Pointer (8 Bytes per smart Pointer)
	// All entities are sorted by their archetype, regardless of if they have a namespace or not.
	archetyped_entities: HashMap<EntityArchetypeKey, Vec<Arc<Mutex<Entity>>>>
}

impl EntityManager {
	pub fn new() -> Self {
		let id_pool = IDPool::new();
		let named_entities = HashMap::<Vec<String>, Vec<Arc<Mutex<Entity>>>>::new();
		let archetyped_entities = HashMap::<EntityArchetypeKey, Vec<Arc<Mutex<Entity>>>>::new();

		EntityManager { id_pool, named_entities, archetyped_entities }
	}

	pub fn add_entity(&mut self, entity: Entity) -> Result<(), Box<dyn ErrorTrait>> {
		let archetype = EntityArchetypeKey::new(entity.get_components()?);
		let arc_entity = Arc::new(Mutex::new(entity));

		// if namespace.len() > 0 {
		// 	if !self.named_entities.contains_key(&namespace) {
		// 		self.named_entities.insert(namespace.clone(), Vec::new());
		// 	}

		// 	let namespace_entities = match self.named_entities.get_mut(&namespace) {
		// 		Some(res) => res,
		// 		None => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to get mutable instance of namespaced entities")))
		// 	};

		// 	namespace_entities.push(Arc::clone(&arc_entity));
		// }

		if !self.archetyped_entities.contains_key(&archetype) {
			self.archetyped_entities.insert(archetype.clone(), Vec::new());
		}

		let archetype_entities = match self.archetyped_entities.get_mut(&archetype) {
			Some(res) => res,
			None => return Err(Box::new(Error::new(ErrorKind::Other, "Failed to get mutable instance of archetype entities")))
		};

		archetype_entities.push(arc_entity);

		Ok(())		
	}

	pub fn allocate_id(&mut self) -> Result<usize, Box<dyn ErrorTrait>> {
		self.id_pool.allocate_id()
	}

	pub fn deallocate_id(&mut self, id: usize) {
		self.id_pool.deallocate_id(id)
	}

	pub fn query_archetyped(&self, components: EntityArchetypeKey) -> Vec<Arc<Mutex<Entity>>> {
		match self.archetyped_entities.get(&components) {
			Some(entities) => entities.clone(),
			None => Vec::new()
		}
	}

	pub fn query_namespaced(&self, namespace: Vec<String>) -> Vec<Arc<Mutex<Entity>>> {
		match self.named_entities.get(&namespace) {
			Some(entities) => entities.clone(),
			None => Vec::new()
		}
	}
}