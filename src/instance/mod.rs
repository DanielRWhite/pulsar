use ahash::{ HashMap, HashMapExt };
use crate::{ connector::Connector, interactor::Interactor };

pub struct Instance<I, C> {
        connectors: Vec<C>,
        interactors: HashMap<String, I>
}

impl<I, C> Instance<I, C>
where
        I: Interactor<Identifier = dyn std::any::Any, Error = dyn std::error::Error>,
        C: Connector<Error = dyn std::error::Error>
{
        pub fn new() -> Instance<I, C> {
                let connectors: Vec<C> = Vec::new();
                let interactors: HashMap<String, I> = HashMap::new();

                Instance { connectors, interactors }
        }
}