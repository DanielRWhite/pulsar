use crate::message::Message;
use tower::Service;
use std::any::Any;

/// A [Message][`crate::message::Message`]-type based router for tower
/// [Service](https://docs.rs/tower/latest/tower/trait.Service.html)s
pub trait Router {
        /// Generic router error type
        type Error;

        /// Call the router with a [Message][`crate::message::Message`],
        /// and let the router decide which [Service](https://docs.rs/tower/latest/tower/trait.Service.html)
        /// the message goes to.
        ///
        /// **Note**: [Service](https://docs.rs/tower/latest/tower/trait.Service.html)s *must* be
        /// thread-safe, and thread-spawnable.
        fn call(&self, message: impl Message) -> Result<(), Self::Error>;
}