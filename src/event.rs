use std::sync::mpsc::Sender;
use std::sync::mpsc::Receiver;
use relay::Relay;
use std::marker::PhantomData;


pub trait Event<Context: Send> {
    fn process(&self, relay: &Relay<Context>, ctx: &mut Context);
}

pub type BoxedEvent<Context> = Box<Event<Context> + Send>;
pub type EventSender<Context> = Sender<BoxedEvent<Context>>;
pub type EventReceiver<Context> = Receiver<BoxedEvent<Context>>;

pub struct GenericEvent<Context, F>
    where F: Fn(&Relay<Context>, &mut Context), Context: Send{
    handler: F,
    context: PhantomData<Context>,
}

impl<Context: Send, F: Fn(&Relay<Context>, &mut Context)> Event<Context> for GenericEvent<Context, F> {
    fn process(&self, relay: &Relay<Context>, ctx: &mut Context) {
        (self.handler)(relay, ctx);
    }
}

pub fn event<Context, F>(handler: F) -> GenericEvent<Context, F>
    where F: Fn(&Relay<Context>, &mut Context), Context: Send {
    GenericEvent {
        handler,
        context: PhantomData,
    }
}