use crate::{ world::World, entity::Entity };
use std::sync::{ Arc, Mutex };

pub trait System {
        fn call(&self, entity: Entity);
}