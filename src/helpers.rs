use event::Event;
use relay::Relay;
use std::marker::PhantomData;

// Generic Event taking closures

#[doc(hidden)]
pub struct GenericEvent<Context, F>
    where F: Fn(&Relay<Context>, &mut Context), Context: Send {
    handler: F,
    context: PhantomData<Context>,
}

impl<Context: Send, F: Fn(&Relay<Context>, &mut Context)> Event<Context> for GenericEvent<Context, F> {
    fn process(self: Box<Self>, relay: &Relay<Context>, ctx: &mut Context) {
        (self.handler)(relay, ctx);
    }
}


/**
An helper function to create an event from a function or a closure

# Example
```
# use bifrost::*;
# struct Context {}
# let mut context = Context {};
# let dispatcher = Dispatcher::new(&mut context);
# let relay = dispatcher.create_relay();
let my_event = event(
    |relay, context| println!("Do something here")
);
# relay.send(my_event)
```
**/
pub fn event<Context, F>(handler: F) -> GenericEvent<Context, F>
    where F: Fn(&Relay<Context>, &mut Context), Context: Send {
    GenericEvent {
        handler,
        context: PhantomData,
    }
}