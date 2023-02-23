pub trait Instance {
        /// This is the type of your [Connector][`crate::connector::Connector`]s.
        /// Usually, this is &[Connector][`crate::connector::Connector`], but
        /// you can set it to whatever you need it to be.
        type ConnectorType;

        /// This is the same as [ConnectorType][`Self::ConnectorType`], but it is
        /// for [Interactor][`crate::interactor::Interactor`]s. Usually this is
        /// set to [Arc][`std::sync::Arc`]<[Mutex][`std::sync::Mutex`]<[Interactor][`crate::interactor::Interactor`]>>
        /// but you can set it to whatever you need it to be.
        type InteractorType;

        /// Generic error type that your instance will respond with when it
        /// encounters an error of any kind.
        type Error;

        /// Get the [Connector][`crate::connector::Connector`]s this 
        /// [Instance][`crate::instance::Instance`] has registered to it.
        fn get_connectors(&self) -> Vec<Self::ConnectorType>;
        
        /// Get the [Interactor][`crate::interactor::Interactor`]s this 
        /// [Instance][`crate::instance::Instance`] has connected to its
        /// registered [Connector][`crate::connector::Connector`]s.
        fn get_interactors(&self) -> Vec<Self::InteractorType>;

        /// Start the instance, serving each [Connector][`crate::connector::Connector`]
        /// in a separate thread, while also starting the game engine.
        fn start(&self) -> Result<(), Self::Error>;
}