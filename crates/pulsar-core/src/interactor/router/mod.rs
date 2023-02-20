use crate::message::Message;
use tower::Service;
use std::any::Any;

pub trait Router {
        type RequestTypes;
        type ResponseTypes;
        type DataTypes;
        type Future;
        type Error;

        // Call the router with a message, and let the router decide which service the message goes to
        fn call(&self, message: impl Message) -> Result<Option<Box<dyn Any>>, Self::Error>;

        fn add_service(&mut self, service: impl Service<Box<dyn Any>, Response = Option<Box<dyn Any>>, Future = Self::Future>) -> Result<(), Self::Error>;
        fn remove_service(&mut self, service: impl Service<Box<dyn Any>, Response = Option<Box<dyn Any>>, Future = Self::Future>) -> Result<(), Self::Error>;
}