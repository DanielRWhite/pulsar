use crate::message::Message;

/// [Poll][`crate::connector::Poll`] is a future, which returns
/// [`Ready<T>`][`crate::connector::Poll::Ready`], [`Pending`][`crate::connector::Poll::Pending`], or [`Error<E>`][`crate::connector::Poll::Error`].
pub enum Poll<T, E> {
        /// Returns `T` when future is done. `T` can be any type.
        Ready(T),

        /// Future has not completed yet, and the result is pending.
        Pending,
        
        /// Future has encounted an error, and an error of type `E` is returned.
        Error(E)
}

/// [Connector][`crate::connector::Connector`]s are just networking implementations that allow
/// [Interactor][`crate::interactor::Interactor`]s to connect to the game server
/// to send & receive [Message][`crate::message::Message`]s.
///
/// Your connector should have its own system of adding/removing Interactors based on
/// connections/disconnections on the connector. These system bounds are not part of the
/// [Connector][`crate::connector::Connector`] trait since there is no single best way to handle
/// how to add/remove interactors for every type of connector, and it is best decided based
/// on the approach your take with your own connector. Usually this is just a simple [HashMap][`std::collections::HashMap`]
/// that uses an [Identifier][`crate::interactor::Identifier`] to store each [Interactor][`crate::interactor::Interactor`] with O(1) efficiency.
pub trait Connector {
        /// The error type your [Connector][`crate::connector::Connector`] will return upon encountering an error of any kind
        type Error;

        /// The raw data type for incoming data to your connector. Used to ensure that all [Interactor][`crate::interactor::Interactor`]s understand
        /// the same [Message][`crate::message::Message`] and implement `From<`[`Self::RawDataType`]`>` that this traits [serve][`crate::connector::Connector::serve`] function
        /// procudes for each incoming piece of data for the specified [Interactor][`crate::interactor::Interactor`].
        type RawDataType;

        /// Connector prepare statement. Pulsar automatically runs
        /// `let connector = YourConnector::prepare().await` in a new thread
        /// and waits for it to return, before invoking `connector.serve().await`
        /// to start the connector
        ///
        /// If the prepare returns an error, it is handled by `YourConnector::handle_error(error)`
        fn prepare() -> Poll<Self, Self::Error> where Self: Sized;

        /// If any error occurs in this trait function, it is best to panic, since it is an unknown errror
        /// and possible malicious in nature, and could alert to more serious issues such as an attacker
        /// attempting to break into the system.
        ///
        /// [`Self::Error`] should be an Enum, or another type, that you can be distinguishable from all possible error
        /// states of your connector. Your connector should be highly available, and handle errors when they occur
        /// rather than stopping & shutting down all clients that are connected to it, regardless of if those clients
        /// caused the error.
        fn handle_error(error: Self::Error);

        /// T is a type that implements [Message][`crate::message::Message`], [`From`][`std::convert::From`]`<`[`Self::RawDataType`]`>` & [Copy][`std::marker::Copy`].
        ///
        /// Message is a generic wrapper of data, that is created in your Connector, and understood by all [Interactor][`crate::interactor::Interactor`]s
        /// that are connected to your [Connector][`crate::connector::Connector`].
        fn serve<T: Message + From<Self::RawDataType> + Copy + 'static>(&self) -> Result<(), Self::Error>;
}