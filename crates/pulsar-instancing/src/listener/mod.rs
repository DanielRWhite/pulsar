use crate::{
        coupler::Coupler,
        connector::Connector
};
use std::error::Error as ErrorTrait;

pub trait Listener: Coupler {
        fn serve(&self, register: dyn Fn(dyn Connector) -> Result<(), Box<dyn ErrorTrait>>) -> Result<(), Box<dyn ErrorTrait>>;
}