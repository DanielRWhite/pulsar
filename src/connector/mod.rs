use crate::{
        message::Message,
        interactor::Interactor
};

pub trait Connector {
        type Error;

        fn recv(&self);
        fn send_to<T, R, I>(&self, to: &I, message: Message<T, R>) -> Result<(), Self::Error>;
        fn broadcast<T, R>(&self, message: Message<T, R>) -> Result<(), Self::Error>;
}