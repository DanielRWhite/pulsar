use ahash::{ HashMap, HashMapExt };
use crate::{ connector::Connector, interactor::{ Interactor, Identifier } };
use std::{
        any::Any,
        sync::{ Arc, Mutex },
        error::Error as ErrorTrait
};

pub struct Instance<C, I> {
        connectors: Vec<Arc<Mutex<C>>>,
        interactors: HashMap<String, Arc<Mutex<I>>>
}

impl<C, I> Instance<C, I>
where
        C: Connector<Error = dyn ErrorTrait>,
        I: Interactor<Error = dyn ErrorTrait> + Identifier<Identifier = dyn Any>
{
        pub fn new() -> Instance<C, I> {
                let connectors: Vec<Arc<Mutex<C>>> = Vec::new();
                let interactors: HashMap<String, Arc<Mutex<I>>> = HashMap::new();

                Instance { connectors, interactors }
        }

        pub fn get_connectors(&self) -> Vec<Arc<Mutex<C>>> {
                self.connectors.iter().map(Arc::clone).collect()
        }

        pub fn get_interactors(&self) -> Vec<Arc<Mutex<I>>> {
                self.interactors.values().map(Arc::clone).collect()
        }

        pub fn get_interactor(&self, name: &str) -> Option<Arc<Mutex<I>>> {
                match self.interactors.get(&name.to_string()) {
                        Some(arc) => Some(Arc::clone(arc)),
                        None => None    
                }
        }
}