/// Blank implementation for a message that is passed throughout the Pulsar engine, originating from the
/// [Connector][`crate::connector::Connector`], which implements a custom [From][`std::convert::From`]
/// for any struct that implements this [Message][`crate::message::Message`] trait.
pub trait Message { }