pub mod router;

use std::any::Any;
use crate::message::Message;

/// An [Identifier][`crate::interactor::Identifier`] is a unique identifier that is used
/// to map [Interactor][`crate::interactor::Interactor`]s.
pub trait Identifier {
        /// The Identifier type that will be returned.
        /// An example is shown in the `DefaultInteractor` which
        /// sets the this to [SocketAddr][`std::net::SocketAddr`].
        type Identifier;

        /// Retreive the unique identifier
        fn get_identifier(&self) -> Self::Identifier;
}

/// An interactor is any external system/user/etc that interacts with the game engine in any way.
///
/// Examples of Interactors are: Other game engine [Instance][`crate::instance::Instance`]s, players, custom hardware (e.g VR Headsets), etc.
pub trait Interactor {
        /// The identifier of the Interactor
        type Identifier: Identifier;

        /// A general error type for your [Interactor][`crate::interactor::Interactor`]
        type Error;

        /// Main interactor loop which is called by Pulsar. This starts your [Interactor][`crate::interactor::Interactor`]
        /// and should listen for incoming messages in this method
        fn listen(&self) -> Result<(), Self::Error>;

        /// Send a [Message][`crate::message::Message`] to the [Interactor][`crate::interactor::Interactor`]
        fn send(&self, message: impl Message) -> Result<usize, Self::Error>;

        /// Handle an error that is related to the [Interactor][`crate::interactor::Interactor`]
        /// instead of stopping the connector entirely.
        ///
        /// This should take [Any][`std::any::Any`] that implements [Into][`std::convert::Into`]<[Self::Error][`Self::Error`]>
        fn handle_error<E: Into<Self::Error>>(&self, data: impl Message, err: E);
}