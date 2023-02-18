use std::{
        any::Any,
        error::Error as ErrorTrait,
        net::{ UdpSocket, ToSocketAddrs }
};
use crate::{
        message::Message,
        interactor::Identifier
};

pub trait Connector: Send + Sync {
        type Error;
        type MessageType;
        type MessageData;

        fn prepare(&self) -> Result<(), Self::Error>;
        fn ready(&self) -> bool;
        fn serve(&self) -> Result<(), Self::Error>;

        fn recv_from(&self, from: &dyn Identifier<Identifier = dyn Any>) -> Result<Message<Self::MessageType, Self::MessageData>, Self::Error>;
        fn send_to(&self, to: &dyn Identifier<Identifier = dyn Any>, message: Message<Self::MessageType, Self::MessageData>) -> Result<usize, Self::Error>;
        fn broadcast(&self, message: Message<Self::MessageType, Self::MessageData>) ->  Result<usize, Self::Error>;
}

pub struct UdpSocketConnector<T, D> {
        message_types: T,
        data_types: D,
        socket: UdpSocket
}

impl<T, D> UdpSocketConnector<T, D> {
        pub fn new<S: ToSocketAddrs<Iter = dyn Any>>(message_types: T, data_types: D, bind_address: S) -> Result<UdpSocketConnector<T, D>, Box<dyn ErrorTrait>> {
                let socket = UdpSocket::bind(bind_address)?;

                Ok(UdpSocketConnector { message_types, data_types, socket })
        }
}

impl<T, D> Connector for UdpSocketConnector<T, D>
where
        T: Send + Sync,
        D: Send + Sync
{
        type Error = Box<dyn ErrorTrait>;
        type MessageType = T;
        type MessageData = D;

        fn prepare(&self) -> Result<(), Self::Error> {
                /// Nothing to prepare, so we just return Ok(())
                Ok(())
        }

        fn ready(&self) -> bool {
                /// Nothing to prepare, so we just return true
                true
        }

        fn serve(&self) -> Result<(), Self::Error> {
                Ok(())
        }

        fn recv_from(&self, from: &dyn Identifier<Identifier = dyn Any>) -> Result<Message<Self::MessageType, Self::MessageData>, Self::Error> {
                todo!()
        }

        fn send_to(&self, to: &dyn Identifier<Identifier = dyn Any>, message: Message<Self::MessageType, Self::MessageData>) -> Result<usize, Self::Error> {
                todo!()
        }

        fn broadcast(&self, message: Message<Self::MessageType, Self::MessageData>) ->  Result<usize, Self::Error> {
                todo!()
        }
}