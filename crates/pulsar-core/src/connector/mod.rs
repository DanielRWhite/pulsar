use std::{
        any::Any,
        error::Error as ErrorTrait
};
use async_trait::async_trait;
use crate::{ interactor::{ Interactor, Identifier }, message::Message };
use tokio::io;

/// `pulsar::connector::Connector`s are just networking implementations that allow
/// `pulsar::interactor::Interactor`s to connect to the game server
/// to send & receive events.
pub trait Connector {
        type Error;
        type Coupler;
        type RawDataType;

        fn prepare(&self) -> Result<(), Self::Error>;
        fn ready(&self) -> bool;

        /// T is a type that implements `pulsar::message::Message`, `std::convert::From<Self::RawDataType>` & `std::marker::Copy`.
        fn serve<T: Message + From<Self::RawDataType> + Copy + 'static>(&self) -> Result<(), Self::Error>;
}