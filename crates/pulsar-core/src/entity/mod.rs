use crate::component::Component;
use std::collections::HashMap;

pub trait EntityTrait { }

pub struct Entity<T> {
        id: String,
        entity: T,
        components: HashMap<String, Box<dyn Component>>
}

impl<T> Entity<T> where T: EntityTrait {
        pub fn new(name: &str, entity: T) -> Self {
                let id = String::from(name);
                let components = HashMap::new();

                Entity { id, entity, components }
        }
}