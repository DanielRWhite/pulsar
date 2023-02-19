use crate::message::Message;
use tower::Service;

pub trait Router {
        type RequestTypes;
        type ResponseTypes;
        type DataTypes;
        type Future;
        type Error;

        // Call the router with a message, and let the router decide which service the message goes to
        fn call(&self, message: Message<Self::RequestTypes, Self::DataTypes>) -> Result<Option<Message<Self::ResponseTypes, Self::DataTypes>>, Self::Error>;

        fn add_service(&mut self, service: impl Service<Message<Self::RequestTypes, Self::DataTypes>, Response = Option<Message<Self::ResponseTypes, Self::DataTypes>>, Future = Self::Future>) -> Result<(), Self::Error>;
        fn remove_service(&mut self, service: impl Service<Message<Self::RequestTypes, Self::DataTypes>, Response = Option<Message<Self::ResponseTypes, Self::DataTypes>>, Future = Self::Future>) -> Result<(), Self::Error>;
}