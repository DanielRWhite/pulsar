pub mod window;
pub mod settings;
pub mod state;
pub mod component;
pub mod entity;

use state::{ State, StateItem };
use std::{
        boxed::Box,
        error::Error as ErrorTrait
};

pub trait PulsarGame {
        fn start(self);
}

pub struct Pulsar {
        state: State,
}

impl Pulsar {
        pub fn new() -> Self {
                let state = State::new();
                
                Pulsar { state }
        }

        pub fn add_to_state(&mut self, key: &str, item: impl StateItem + 'static) -> Result<(), Box<dyn ErrorTrait>> {
                self.state.add_to_state(key, item)
        }

        pub fn get_item<F: FnOnce(&Box<dyn StateItem>) -> ()>(&self, key: &str, closure: F) -> Result<(), Box<dyn std::error::Error>> {
                self.state.get_item(key, closure)
        }
}