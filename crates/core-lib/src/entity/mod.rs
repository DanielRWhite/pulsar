#[cfg(test)]
mod tests;

mod archetype;

use archetype::Archetype;
use ahash::{AHasher, RandomState};
use crate::component::Component;
use std::{
        collections::HashMap,
        sync::{ Arc, Mutex }
};

pub struct EntityManager {
        entities: HashMap<Archetype, Vec<Arc<Mutex<Entity>>>, RandomState>,
        available_ids: Vec<usize>,
        next_id: usize
}

impl EntityManager {
        pub fn new() -> EntityManager {
                let entities: HashMap<Archetype, Vec<Arc<Mutex<Entity>>>, RandomState> = HashMap::default();
                let available_ids: Vec<usize> = Vec::new();
                let next_id: usize = 0;

                EntityManager { entities, available_ids, next_id }
        }

        /* START ENTITY ID MANAGEMENT */
        fn reclaim_id(&mut self, id: usize) -> Option<()> {
                if let Some(pos) = self.available_ids.iter().position(|x| x == &id) {
                        None
                } else {
                        if id == self.next_id {
                                self.next_id = self.next_id - 1;
                                Some(())
                        } else {
                                self.available_ids.push(id);
                                Some(())
                        }
                }
        }

        fn reserve_id(&mut self) -> Result<usize, String> {
                if self.available_ids.len() > 0 {
                        Ok(self.available_ids.pop().unwrap())
                } else {
                        if self.next_id + 1 > usize::MAX {
                                Err(String::from("Maximum ID count reached"))
                        } else {
                                let id = self.next_id;
                                self.next_id = self.next_id + 1;

                                Ok(id)
                        }
                }
        }

        fn cleanup_ids(&mut self) {
                let max_entity_id = self.entities.values().fold(0, |o_acc, v_e| {
                        let entity_id = v_e.iter().fold(0, |acc, e| {
                                let mu = match Arc::try_unwrap(e.clone()) {
                                        Ok(mu) => mu,
                                        Err(_) => return acc
                                };
        
                                let ent = match mu.lock() {
                                        Ok(en) => en,
                                        Err(_) => return acc
                                };
        
                                let id = ent.get_id();
                                if id > acc {
                                        id
                                } else {
                                        acc
                                }
                        });

                        if entity_id > o_acc {
                                entity_id
                        } else {
                                o_acc
                        }
                });

                if max_entity_id + 1 < usize::MAX {
                        self.next_id = max_entity_id + 1;
                }

                let ids: Vec<usize> = self.available_ids.iter().enumerate().filter_map(|(index, x)| {
                        if *x > max_entity_id {
                                Some(index)
                        } else {
                                None
                        }
                }).collect();

                for index in ids {
                        self.available_ids.swap_remove(index);
                }

        }
        /* END ENTITY ID MANAGEMENT */

        pub fn num_entities(&self) -> usize {
                self.entities.iter().fold(0 as usize, |acc, (_, vec)| acc + vec.len())
        }
}

pub struct Entity {
        components: Vec<Box<dyn Component>>
}

impl Entity {
        fn get_id(&self) -> usize {
                0
        }
}