use ahash::{ HashMap, HashMapExt };
use pulsar::{
        message::Message,
        interactor::{ Interactor, Identifier },
        connector::Connector
};
use async_trait::async_trait;
use std::{
        io::{ Error, ErrorKind },
        error::Error as ErrorTrait,
        sync::mpsc::Sender,
        net::{ UdpSocket, SocketAddr },
        hash::Hash,
        any::Any,
};

pub struct UDPSocketConnector<In> {
        addr: SocketAddr,
        socket: UdpSocket,
        interactors: HashMap<SocketAddr, (In, Sender<Box<dyn Message>>)>
}

impl<In> UDPSocketConnector<In> 
where
        In: Interactor + Identifier<Identifier = SocketAddr>
{
        pub async fn new(addr: SocketAddr) -> Result<UDPSocketConnector<In>, std::io::Error> {
                let socket = UdpSocket::bind(addr)?;
                let interactors: HashMap<SocketAddr, (In, Sender<Box<dyn Message>>)> = HashMap::new();

                Ok(UDPSocketConnector { addr, socket, interactors })
        }

        fn add_interactor(&mut self, interactor: In, coupler: Sender<Box<dyn Message>>) -> Result<(), Box<dyn ErrorTrait>> {
                let identifier = interactor.get_identifier();
                
                if self.interactors.contains_key(&identifier) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Interactor with that identifier already exists on this connector")))
                }

                self.interactors.insert(identifier, (interactor, coupler));

                Ok(())
        }

        fn delete_interactor(&mut self, interactor: In) -> Result<(), Box<dyn ErrorTrait>> {
                let identifier = interactor.get_identifier();
                
                if !self.interactors.contains_key(&identifier) {
                        return Err(Box::new(Error::new(ErrorKind::Other, "Interactor with that does not exist on this connector")))
                }

                self.interactors.remove(&identifier);

                Ok(())
        }
}

impl<In> Connector for UDPSocketConnector<In>
where
        In: Interactor<Error = dyn ErrorTrait> + Identifier<Identifier = SocketAddr>
{
        type Error = Box<dyn ErrorTrait>;
        type Coupler = Sender<Box<dyn Message>>;
        type RawDataType = Vec<u8>;

        fn prepare(&self) -> Result<(), Self::Error> {
                // Nothing to prepare
                Ok(())
        }

        fn ready(&self) -> bool {
                // Nothing to be ready for
                true
        }

        /// T is a type that implements `pulsar::message::Message`, `std::convert::From<Self::RawDataType>` & `std::marker::Copy`.
        /// `Self::RawDataType` in this case is `Vec<u8>`.
        fn serve<T: Message + From<Self::RawDataType> + Copy + 'static>(&self) -> Result<(), Self::Error> {
                loop {
                        let mut buf: Vec<u8> = Vec::new();
                        let (len, addr) = self.socket.recv_from(&mut buf)?;

                        match self.interactors.get(&addr) {
                                Some((interactor, sender)) => {
                                        let message = <T>::from(buf);
                                        match sender.send(Box::new(message)) {
                                                Ok(()) => { },
                                                Err(err) => interactor.handle_error(message, Box::new(err))
                                        }
                                },
                                None => { }
                        }
                }
        }
}