use pulsar_instancing::{
        connector::Connector,
        listener::Listener,
};
use std::{
        error::Error as ErrorTrait,
        io::{ Error, ErrorKind },
        sync::{ Arc, Mutex }
};
use ahash::{ HashMap, HashMapExt };

pub struct InstanceBuilder {
        listeners: HashMap<String, Arc<Mutex<dyn Listener>>>,
        connectors: HashMap<String, Arc<Mutex<dyn Connector>>>
}

impl InstanceBuilder {
        pub fn new() -> InstanceBuilder {
                let listeners: HashMap<String, Arc<Mutex<dyn Listener>>> = HashMap::new();
                let connectors: HashMap<String, Arc<Mutex<dyn Connector>>> = HashMap::new();

                InstanceBuilder { listeners, connectors }
        }

        pub fn add_listener<L: Listener + 'static>(mut self, listener: L) -> Result<Self, Box<dyn ErrorTrait>> {
                let name: String = listener.get_name();
                if self.listeners.contains_key(&name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Listener with that name already exists")))
                }

                self.listeners.insert(name, Arc::new(Mutex::new(listener)));

                Ok(self)
        }

        pub fn build(self) -> Instance {
                Instance::new(self.listeners, self.connectors)
        }
}

pub struct Instance {
        listeners: HashMap<String, Arc<Mutex<dyn Listener>>>,
        connectors: HashMap<String, Arc<Mutex<dyn Connector>>>
}

impl Instance {
        pub fn new(
                listeners: HashMap<String, Arc<Mutex<dyn Listener>>>,
                connectors: HashMap<String, Arc<Mutex<dyn Connector>>>
        ) -> Instance {
                Instance { listeners, connectors }
        }

        pub fn builder() -> InstanceBuilder {
                InstanceBuilder::new()
        }

        pub fn register_connector<C: Connector + 'static>(&mut self, connector: C) -> Result<(), Box<dyn ErrorTrait>> {
                let name: String = connector.get_name();
                if self.connectors.contains_key(&name) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Connector with that name already exists")))
                }

                self.connectors.insert(name, Arc::new(Mutex::new(connector)));
                
                Ok(())
        }
}