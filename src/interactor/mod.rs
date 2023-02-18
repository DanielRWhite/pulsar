pub mod router;

pub trait Identifier {
        type Identifier;

        fn get_identifier(&self) -> Self::Identifier;
}

pub trait Interactor {
        type Error;

        fn recv(&self) -> Result<(), Self::Error>;
        fn send(&self) -> Result<(), Self::Error>;
}