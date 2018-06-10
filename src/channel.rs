use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;

pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

impl<T> Channel<T> {
    pub fn new() -> Channel<T> {
        let (sender, receiver) = mpsc::channel();
        Channel {
            sender,
            receiver,
        }
    }

    pub fn get_sender(&self) -> &Sender<T> {
        &self.sender
    }

    pub fn get_receiver(&self) -> &Receiver<T> {
        &self.receiver
    }
}
