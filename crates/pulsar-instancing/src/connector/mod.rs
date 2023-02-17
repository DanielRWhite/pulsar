use crate::coupler::Coupler;
use std::{
        error::Error as ErrorTrait,
        any::Any
};

pub trait Connector: Coupler + Send + Sync {
        fn connect(&self) -> Result<Box<dyn Any>, Box<dyn ErrorTrait>>;
        fn disconnect(&self) -> Result<Box<dyn Any>, Box<dyn ErrorTrait>>;
        fn is_connected(&self) -> bool;

        // Send a message to the other instance, where T is an enum with message types
        fn send(&self, data: Vec<u8>) -> Result<Box<dyn Any>, Box<dyn ErrorTrait>>;
}