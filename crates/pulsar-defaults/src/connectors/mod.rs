pub mod udp;

// use pulsar::{
//         message::{ Message, MessageCreator },
//         interactor::{ Interactor, Identifier },
//         connector::{ Coupler, Connector }
// };
// use crate::coupler::DefaultCoupler;
// use async_trait::async_trait;
// use tokio::{ io, net::UdpSocket };
// use std::{
//         cmp::{ Eq, PartialEq },
//         hash::Hash,
//         any::Any,
//         net::SocketAddr,
//         error::Error as ErrorTrait,
//         sync::mpsc::Sender
// };
// use ahash::{ HashMap, HashMapExt };

// pub struct UdpSocketConnector<Id, In> {
//         addr: SocketAddr,
//         socket: UdpSocket,
//         interactors: HashMap<Id, (In, Sender<Box<dyn Any>>)>
// }

// impl<Id, In> UdpSocketConnector<Id, In> {
//         pub async fn new(addr: SocketAddr) -> io::Result<UdpSocketConnector<Id, In>> {
//                 let socket = UdpSocket::bind(addr).await?;
//                 let interactors: HashMap<Id, (In, Sender<()>)> = HashMap::new();

//                 Ok(UdpSocketConnector { addr, socket, interactors })
//         }
// }

// #[async_trait]
// impl<Id, In> Connector for UdpSocketConnector<Id, In>
// where
//         Id: Identifier<Identifier = dyn Any> + From<SocketAddr> + Hash + Eq + PartialEq,
//         In: Interactor<RequestTypes = dyn Any, ResponseTypes = dyn Any, DataTypes = dyn Any, Future = dyn Any, Error = dyn ErrorTrait> + Identifier<Identifier = dyn Any>
// {
//         type Error = Box<dyn ErrorTrait>;
//         type Coupler = DefaultCoupler;

//         fn prepare(&self) -> Result<(), Self::Error> {
//                 // Nothing to prepare
//                 Ok(())
//         }

//         fn ready(&self) -> bool {
//                 // Nothing to prepare
//                 true
//         }

//         async fn serve(&self) -> io::Result<()> {
//                 loop {
//                         let mut buf: Vec<u8> = Vec::new();
//                         let (len, addr) = self.socket.recv_from(&mut buf).await?;

//                         let identifier = Id::from(addr);
//                         match self.interactors.get(&identifier) {
//                                 Some((interactor, sender)) => {
//                                         let message = interactor.create_message(buf);
//                                         match sender.send(message) {
//                                                 Ok(()) => { },
//                                                 Err(err) => interactor.handle_error(message, Box::new(err))
//                                         }
//                                 },
//                                 None => { }
//                         };
//                 }
//         }

//         fn add_interactor<I>(&mut self, interactor: I) -> Result<Self::Coupler, Self::Error> {
//                 todo!()
//         }

//         fn delete_interactor<I>(&mut self, interactor: I) -> Result<(), Self::Error> {
//                 todo!()
//         }
// }