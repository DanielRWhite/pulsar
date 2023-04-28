#[cfg(all(feature = "controller", feature = "listener"))]
compile_error!("Please ensure that you are only enabling the feature for one of the two mutually exclusive instance types, not both: Controller & Listener.");

#[cfg(not(feature = "core"))]
compile_error!("The pulsar feature \"core\" must be enabled at the very minimum for this package to compile.");

#[cfg(feature = "controller")]
mod prelude {
        pub use core::{
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
        pub use core::{
                component::Component,
                entity::Entity,
                system::System,
                world::World,
        };
        pub use listener::Listener;

        #[cfg(feature = "accelerator")]
        pub use accelerator::Accelerator;
}

#[cfg(all(not(feature = "controller"), not(feature = "listener"), feature = "core"))]
mod prelude {
        pub use core::{
                component::Component,
                entity::Entity,
                system::System,
                world::World,
        };
        
        #[cfg(feature = "accelerator")]
        pub use accelerator::Accelerator;
}