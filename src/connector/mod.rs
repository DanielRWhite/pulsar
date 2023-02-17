pub trait Connector<Request, I, O> {
        type Error;

        fn request_type(&self, request: &Request) -> Result<I, Self::Error>;
        fn call(&self, request: Request, request_type: I) -> Result<O, Self::Error>;
}