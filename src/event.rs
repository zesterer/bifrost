use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use relay::Relay;


pub trait Event<Context: Send> {
    fn process(&self, relay: &Relay<Context>, ctx: &mut Context);
}

pub type BoxedEvent<Context> = Box<Event<Context> + Send>;
pub type EventSender<Context> = Sender<BoxedEvent<Context>>;
pub type EventReceiver<Context> = Receiver<BoxedEvent<Context>>;