pub mod router;

use tower::Service;
use crate::message::Message;

pub trait Identifier {
        type Identifier;

        fn get_identifier(&self) -> Self::Identifier;
}

pub trait Interactor {
        type Identifier;
        type RequestTypes;
        type ResponseTypes;
        type DataTypes;
        type Future;
        type Error;

        fn get_identifier(&self) -> Self::Identifier;

        // Listen for incoming requests
        fn listen(&self) -> Result<(), Self::Error>;

        // Send response to interactor
        fn send(&self, message: Message<Self::ResponseTypes, Self::DataTypes>) -> Result<usize, Self::Error>;
        
        // Router
        fn add_service(&mut self, service: impl Service<Message<Self::RequestTypes, Self::DataTypes>, Response = Option<Message<Self::ResponseTypes, Self::DataTypes>>, Future = Self::Future>) -> Result<(), Self::Error>;
        fn remove_service(&mut self, service: impl Service<Message<Self::RequestTypes, Self::DataTypes>, Response = Option<Message<Self::ResponseTypes, Self::DataTypes>>, Future = Self::Future>) -> Result<(), Self::Error>;
}