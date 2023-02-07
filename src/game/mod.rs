use crate::entity::Entity;
use std::{
        boxed::Box,
        collections::HashMap,
        error::Error as ErrorTrait,
        io::{ Error, ErrorKind }
};

pub struct PulsarGame {
        entity_states: HashMap<String, Vec<Entity>>,
        num_entities: usize,
}

impl PulsarGame {
        pub fn new() -> Self {
                let entity_states = HashMap::new();
                let num_entities = 0;

                PulsarGame { entity_states, num_entities }
        }
        
        pub fn create_entity_state(&mut self, name: &str) -> Result<(), Box<dyn ErrorTrait>> {
                if self.entity_states.contains_key(name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state already exists")))
                }

                self.entity_states.insert(name.to_string(), Vec::new());

                Ok(())
        }

        pub fn remove_entity_state(&mut self, name: &str) -> Result<(), Box<dyn ErrorTrait>> {
                if !self.entity_states.contains_key(name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state doesn't exist")))
                }

                if let Some(state) = self.entity_states.get(name) {
                        if state.len() > 0 {
                                return Err(Box::new(Error::new(ErrorKind::Other, "Entity state isn't empty")))
                        }

                        self.entity_states.remove(name);
                };

                Ok(())
        }

        pub fn get_entity_states(&self) -> Vec<String> {
                self.entity_states.keys().map(String::from).collect::<Vec<String>>()
        }

        pub fn clear_all_entity_states(&mut self) {
                self.entity_states = HashMap::new();
                self.num_entities = 0;
        }

        pub fn get_entities_in_state(&self, name: &str) -> Result<Vec<Entity>, Box<dyn ErrorTrait>> {
                if !self.entity_states.contains_key(name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state doesn't exist")))
                }

                if let Some(state) = self.entity_states.get(name) {
                        Ok(state.clone())
                } else {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state is None")))
                }
        }

        pub fn get_state_of_entity(&self, id: usize) -> Result<String, Box<dyn ErrorTrait>> {
                for key in self.entity_states.keys().map(String::from).collect::<Vec<String>>() {
                        let entity_state = match self.entity_states.get(&key) {
                                Some(res) => res,
                                None => continue
                        };

                        if let Some(_) = entity_state.iter().position(|entity| entity.0 == id) {
                                return Ok(String::from(&key))
                        }
                }

                return Err(Box::new(Error::new(ErrorKind::Other, "Entity not found")))
        }

        pub fn set_entity_state(&mut self, id: usize, new_state: &str, old_state: Option<&str>) -> Result<(), Box<dyn ErrorTrait>> {
                if !self.entity_states.contains_key(new_state) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state doesn't exist")))
                }

                let entity = self.remove_entity(id, old_state)?;
                match self.entity_states.get_mut(new_state) {
                        Some(res) => {
                                res.push(entity);
                                Ok(())
                        },
                        None => Err(Box::new(Error::new(ErrorKind::Other, "Entity state is None")))
                }
        }

        pub fn create_entity(&mut self, state_name: &str) -> Result<usize, Box<dyn ErrorTrait>> {
                if !self.entity_states.contains_key(state_name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Entity state doesn't exist")))
                }

                if self.num_entities + 1 > usize::MAX {
                        return Err(Box::new(Error::new(ErrorKind::Other, format!("Maximum amount of allowed entities reached: {}", usize::MAX))))
                }

                let entity_id = self.num_entities + 1;
                let entity = Entity(entity_id);

                match self.entity_states.get_mut(state_name) {
                        Some(res) => {
                                res.push(entity);
                                self.num_entities = self.num_entities + 1;
                                Ok(entity_id)
                        },
                        None => Err(Box::new(Error::new(ErrorKind::Other, "Entity state is None")))
                }
        }

        pub fn remove_entity(&mut self, id: usize, state_name: Option<&str>) -> Result<Entity, Box<dyn ErrorTrait>> {
                if let Some(state) = state_name {
                        if !self.entity_states.contains_key(state) {
                                return Err(Box::new(Error::new(ErrorKind::Other, "Entity state doesn't exist")))
                        }

                        let entity_state = match self.entity_states.get_mut(state) {
                                Some(res) => res,
                                None => return Err(Box::new(Error::new(ErrorKind::Other, "Entity state is None")))
                        };

                        if let Some(index) = entity_state.iter().position(|entity| entity.0 == id) {
                                // Fast remove in O(1) time, by not preserving vec item order and replacing
                                // the index that we removed with the last item in the last index in the vec
                                let entity = entity_state.swap_remove(index);
                                self.num_entities = self.num_entities - 1;
                                return Ok(entity)
                        } else {
                                return Err(Box::new(Error::new(ErrorKind::Other, "Entity ID not found in given state")))
                        }
                }

                for key in self.entity_states.keys().map(String::from).collect::<Vec<String>>() {
                        let entity_state = match self.entity_states.get_mut(&key) {
                                Some(res) => res,
                                None => continue
                        };

                        if let Some(index) = entity_state.iter().position(|entity| entity.0 == id) {
                                // Fast remove in O(1) time, by not preserving vec item order and replacing
                                // the index that we removed with the last item in the last index in the vec
                                let entity = entity_state.swap_remove(index);
                                self.num_entities = self.num_entities - 1;
                                return Ok(entity)
                        }
                }

                return Err(Box::new(Error::new(ErrorKind::Other, "Entity ID not found in any entity state")))
        }
}