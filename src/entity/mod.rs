use crate::{
	world::World,
	component::Component
};

use std::{
	collections::HashMap,
	io::{ Error, ErrorKind },
	sync::{ Arc, Mutex }
};

pub struct EntityBuilder {
	pub components: Vec<Arc<Mutex<Box<dyn Component>>>>,
	pub archetype: Vec<String>,
	pub systems: Vec<String>,
	pub categories: Vec<String>
}

#[derive(Clone)]
pub struct Entity {
	id: usize,
	components: HashMap<String, Arc<Mutex<Box<dyn Component>>>>,
	categories: Vec<String>
}

impl Entity {
	pub fn new(id: usize) -> Entity {
		let components = HashMap::<String, Arc<Mutex<Box<dyn Component>>>>::new();
		let categories = Vec::<String>::new();

		Entity { id, components, categories }
	}

	pub fn from_builder(id: usize, builder: EntityBuilder) -> Entity {
		let mut components = HashMap::<String, Arc<Mutex<Box<dyn Component>>>>::new();
		builder.archetype.into_iter().zip(builder.components.into_iter()).for_each(|(component_type, component)| { components.insert(component_type, component); });

		Entity {
			id,
			// We don't clone here, instead we pass ownership of the HashMap and all
			// Arc<Mutex<dyn Component>> to the new Entity.
			components,

			// The ownership of all the Strings in the Vec will be passed to the Entity
			// once this function returns and builder goes out of scope.
			categories: builder.categories
		}
	}

	pub fn get_id(&self) -> usize {
		self.id.clone()
	}

	pub fn get_component(&self, component_name: &str) -> Option<Arc<Mutex<Box<dyn Component>>>> {
		match self.components.get(component_name) {
			Some(res) => Some(res.clone()),
			None => None
		}
	}

	pub fn get_archetype(&self) -> Vec<String> {
		self.components.keys().map(|x| x.to_string()).collect()
	}

	pub fn get_components(&self) -> HashMap<String, Arc<Mutex<Box<dyn Component>>>> {
		self.components.clone()
	}
}

impl PartialEq for Entity {
	fn eq(&self, other: &Self) -> bool {
		self.id == other.id
	}
}

impl EntityBuilder {
	pub fn new() -> EntityBuilder {
		EntityBuilder {
			components: Vec::<Arc<Mutex<Box<dyn Component>>>>::new(),
			archetype: Vec::<String>::new(),
			systems: Vec::<String>::new(),
			categories: Vec::<String>::new(),
		}
	}

	pub fn add_category(mut self, category: &str) -> Self {
		let lower = category.to_lowercase();
		if !self.categories.contains(&lower) {
			self.categories.push(lower);
		}

		self
	}

	pub fn add_component<'a, C: Component + Sized + 'static>(mut self, component: C) -> Result<Self, Box<dyn std::error::Error>> {
		if !self.archetype.contains(&component.get_type()) {
			self.archetype.push(component.get_type());
			self.components.push(Arc::new(Mutex::new(Box::new(component))));
			Ok(self)
		} else {
			Err(Box::new(Error::new(ErrorKind::Other, "Entity already contains component of this type")))
		}
	}

	pub fn build(self, world: &mut World) -> Result<Entity, Box<dyn std::error::Error>> {
		if self.components.len() == self.archetype.len() {
			Ok(Entity::from_builder(world.reserve_entity_id()?, self))
		} else {
			Err(Box::new(Error::new(ErrorKind::Other, "Mismatch between length of component types & components, please make sure you are adding components with builder.add_component(component)")))
		}
	}
}