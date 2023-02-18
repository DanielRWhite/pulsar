use crate::message::Message;

pub trait Router {
        type MessageData;
        type Requests;
        type Responses;
        type Error;

        fn call(&self, message: Message<Self::Requests, Self::MessageData>)
                -> Result<
                        Option<Message<Self::Responses, Self::MessageData>>,
                        Self::Error
                >;
}