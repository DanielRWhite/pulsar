#[cfg(all(feature = "controller", feature = "listener"))]
compile_error!("Please ensure that you are only enabling the feature for one of the two mutually exclusive instance types, not both: Controller & Listener.");

#[cfg(not(feature = "stdlib"))]
compile_error!("The pulsar feature \"stdlib\" must be enabled at the very minimum for this package to compile.");

#[cfg(feature = "controller")]
mod prelude {
        pub use stdlib::{
                component::Component,
                entity::Entity,
                system::System,
                world::World,
        };
        pub use controller::Controller;

        #[cfg(feature = "accelerator")]
        pub use accelerator::Accelerator;
}

#[cfg(feature = "listener")]
mod prelude {
        pub use stdlib::{ Entity, Component, System, World };
        pub use listener::Listener;

        #[cfg(feature = "accelerator")]
        pub use accelerator::Accelerator;
}

#[cfg(all(not(feature = "controller"), not(feature = "listener"), feature = "stdlib"))]
mod prelude {
        pub use stdlib::{ Entity, Component, System, World };

        #[cfg(feature = "accelerator")]
        pub use accelerator::Accelerator;
}