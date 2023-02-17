use crate::message::Message;

pub trait Interactor {
        type Identifier;
        type Error;

        fn get_identifier(&self) -> Self::Identifier;

        fn recv<T, R>(&self, message: Message<T, R>) -> Result<(), Self::Error>;
        fn send<T, R>(&self, message: Message<T, R>) -> Result<(), Self::Error>;
}