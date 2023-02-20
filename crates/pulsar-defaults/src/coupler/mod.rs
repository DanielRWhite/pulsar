use std::error::Error as ErrorTrait;
use std::sync::mpsc::{ Receiver, Sender, channel };

pub struct DefaultCoupler {
        sender: Sender<()>,
        receiver: Receiver<()>,
}

impl DefaultCoupler {
        pub fn new() -> DefaultCoupler {
                let (sender, receiver) = channel();
                
                DefaultCoupler { receiver, sender }
        }

        pub fn clone(&self) -> Sender<()> {
                self.sender.clone()
        }
}