use pulsar::{
        interactor::{ Interactor, Identifier },
        connector::{ Coupler, Connector }
};
use crate::coupler::DefaultCoupler;
use std::any::Any;
use std::net::{ UdpSocket, SocketAddr };
use std::error::Error as ErrorTrait;
use ahash::{ HashMap, HashMapExt };

pub struct UdpSocketConnector<Id, In> {
        addr: SocketAddr,
        socket: UdpSocket,
        interactors: HashMap<Id, In>
}

impl<Id, In> UdpSocketConnector<Id, In> {
        pub fn new(addr: SocketAddr) -> std::io::Result<UdpSocketConnector<Id, In>> {
                let socket = UdpSocket::bind(addr)?;
                let interactors: HashMap<Id, In> = HashMap::new();

                Ok(UdpSocketConnector { addr, socket, interactors })
        }
}

impl<Id, In> Connector for UdpSocketConnector<Id, In>
where
        Id: Identifier<Identifier = dyn Any>,
        In: Interactor<Error = dyn ErrorTrait>
{
        type Error = Box<dyn ErrorTrait>;
        type Coupler = DefaultCoupler;

        fn prepare(&self) -> Result<(), Self::Error> {
                // Nothing to prepare
                Ok(())
        }

        fn ready(&self) -> bool {
                // Nothing to prepare
                true
        }

        fn serve(&self) -> Result<(), Self::Error> {
                todo!()
        }

        fn add_interactor<I>(&mut self, interactor: I) -> Result<Self::Coupler, Self::Error> {
                todo!()
        }

        fn delete_interactor<I>(&mut self, interactor: I) -> Result<(), Self::Error> {
                todo!()
        }
}