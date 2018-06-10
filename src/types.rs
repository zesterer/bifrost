use event::Event;
use std::sync::mpsc::Sender;

pub type BoxedEvent<Context> = Box<Event<Context> + Send>;
pub type EventSender<Context> = Sender<BoxedEvent<Context>>;