/// Blank implementation for a message that is passed throughout the Pulsar engine, originating from the
/// Connector, which implements a custom `From<CustomDataType>` for any struct that implements this
/// `pulsar_core::message::Message` trait.
pub trait Message { }