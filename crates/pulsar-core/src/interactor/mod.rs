pub mod router;

use std::any::Any;
use tower::Service;
use crate::message::Message;

/// An `pulsar::interactor::Identifier` is a unique identifier that is used
/// to map `pulsar::interactor::Interactor`s.
pub trait Identifier {
        type Identifier;

        fn get_identifier(&self) -> Self::Identifier;
}

pub trait Interactor: Identifier {
        type Identifier;
        type RequestTypes;
        type ResponseTypes;
        type DataTypes;
        type Future;
        type Error;

        // Listen for incoming requests
        fn listen(&self) -> Result<(), Self::Error>;

        // Send response to interactor
        fn send(&self, message: impl Message) -> Result<usize, Self::Error>;
        
        // Router
        fn add_service(&mut self, service: impl Service<Box<dyn Message>, Response = Option<Self::ResponseTypes>, Future = Self::Future>) -> Result<(), Self::Error>;
        fn remove_service(&mut self, service: impl Service<Box<dyn Message>, Response = Option<Self::ResponseTypes>, Future = Self::Future>) -> Result<(), Self::Error>;

        // Error
        fn handle_error(&self, data: impl Message, err: Box<dyn Any>);
}