use std::error::Error as ErrorTrait;
use crate::interactor::Interactor;

pub trait Coupler { }

pub trait Connector {
        type Error;
        type Coupler;

        fn prepare(&self) -> Result<(), Self::Error>;
        fn ready(&self) -> bool;
        fn serve(&self) -> Result<(), Self::Error>;

        fn add_interactor<I>(&mut self, interactor: I) -> Result<Self::Coupler, Self::Error>;
        fn delete_interactor<I>(&mut self, interactor: I) -> Result<(), Self::Error>;
}