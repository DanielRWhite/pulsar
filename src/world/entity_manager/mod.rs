pub mod keys;

use crate::entity::Entity;
use keys::EntityArchetypeKey;
use ahash::{ HashMap, HashMapExt };
use std::{
        sync::{ Arc, Mutex },
        io::{ Error, ErrorKind },
        error::Error as ErrorTrait
};

pub struct IDPool {
        current_id: usize,
        available_ids: Vec<usize>,
        used_ids: Vec<usize>
}

impl IDPool {
        pub fn new() -> Self {
                let current_id = usize::MIN;
                let available_ids: Vec<usize> = Vec::new();
                let used_ids: Vec<usize> = Vec::new();

                IDPool { current_id, available_ids, used_ids }
        }

        pub fn register_id(&mut self) -> Result<usize, Box<dyn ErrorTrait>> {
                if self.available_ids.len() > 0 {
                        let id = self.available_ids.pop().unwrap();
                        self.used_ids.push(id.clone());
                        return Ok(id)
                }

                if self.current_id == usize::MAX {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Maximum amount of entities reached")))
                }

                self.current_id = self.current_id + 1;
                self.used_ids.push(self.current_id.clone());
                return Ok(self.current_id.clone())
        }

        pub fn deregister_id(&mut self, id: usize) -> Result<(), Box<dyn ErrorTrait>> {
                if let Some(pos) = self.used_ids.iter().position(|x| x == &id) {
                        self.used_ids.remove(pos);
                        self.available_ids.push(id);
                }

                Ok(())
        }

        pub fn housekeeping(&mut self) { }
}

pub struct EntityManager {
        ids: IDPool,
        entities: HashMap<usize, Arc<Mutex<Entity>>>,
        archetypes: HashMap<EntityArchetypeKey, HashMap<usize, Arc<Mutex<Entity>>>>
}

impl EntityManager {
        pub fn new() -> Self {
                let ids: IDPool = IDPool::new();
                let entities: HashMap<usize, Arc<Mutex<Entity>>> = HashMap::new();
                let archetypes: HashMap<EntityArchetypeKey, HashMap<usize, Arc<Mutex<Entity>>>> = HashMap::new();

                EntityManager { ids, entities, archetypes }
        }

        pub fn create_entity(&mut self, entity: Entity) -> Result<(), Box<dyn ErrorTrait>> {
                let archetype = entity.get_archetype();
                let entity_id = entity.get_id();

                if let Some(_) = self.entities.get(&entity_id) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity with that ID already exists")))
                }

                if !self.archetypes.contains_key(&archetype) {
                        self.archetypes.insert(archetype.clone(), HashMap::new());
                }

                // Safe unwrap since we made sure that it exists in the above block
                let entities = self.archetypes.get_mut(&archetype).unwrap();
                let entity = Arc::new(Mutex::new(entity));
                entities.insert(entity_id.clone(), Arc::clone(&entity));
                
                self.entities.insert(entity_id, entity);

                Ok(())
        }

        pub fn delete_entity(&mut self, id: usize, archetype: Option<EntityArchetypeKey>) -> Result<(), Box<dyn ErrorTrait>> {
                if !self.entities.contains_key(&id) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity doesn't exist")))
                }

                if let Some(arch) = archetype {
                        if !self.archetypes.contains_key(&arch) {
                                return Err(Box::new(Error::new(ErrorKind::Other, "Archetype doesn't exist")))
                        }

                        // Safe unwrap since we checked if it contains this key above
                        let entities = self.archetypes.get_mut(&arch).unwrap();
                        entities.remove(&id);
                        self.entities.remove(&id);

                        return Ok(())
                } else {
                        for (_, archetype) in self.archetypes.iter_mut() {
                                if archetype.contains_key(&id) {
                                        archetype.remove(&id);
                                        self.entities.remove(&id);

                                        return Ok(())
                                }
                        }
                }

                return Err(Box::new(Error::new(ErrorKind::Other, "Entity not found")))
        }

        pub fn get_entity(&self, id: usize) -> Result<Arc<Mutex<Entity>>, Box<dyn ErrorTrait>> {
                if !self.entities.contains_key(&id) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity doesn't exist")))
                }

                Ok(Arc::clone(&self.entities.get(&id).unwrap()))
        }

        pub fn query(&self, archetype: EntityArchetypeKey) -> Vec<Arc<Mutex<Entity>>> {
                let mut entities: Vec<Arc<Mutex<Entity>>> = Vec::new();
                for (key, archetyped) in self.archetypes.iter() {
                        if key.include_in_query(&archetype) {
                                archetyped.values().for_each(|x| entities.push(Arc::clone(x)));
                        }
                }

                entities
        }

        pub fn register_id(&mut self) -> Result<usize, Box<dyn ErrorTrait>> {
                self.ids.register_id()
        }

        pub fn deregister_id(&mut self, id: usize) -> Result<(), Box<dyn ErrorTrait>> {
                self.ids.deregister_id(id)
        }
}