use tower::Service;

pub trait Interactor<Request> {
	type Identifier;
	type Error;

	fn get_identifier(&self) -> Self::Identifier;
	fn call(&self, request: Request);
	fn send<Response>(&self, response: Response) -> Result<(), Self::Error>;
}