use command::Command;
use event::Event;
use std::sync::mpsc::Sender;
use std::time::Duration;
use types::EventSender;

/**
A structure used to send events and commands to a Dispatcher.

Relays are lightweight, cloneable and can be sent across threads and channels

You can get a relay from a Dispatcher with the `create_relay` method
```
# use bifrost::*;
# struct Context {}
# let mut context = Context {};
# let dispatcher = Dispatcher::new(&mut context);

let relay = dispatcher.create_relay();
```
Or by cloning another relay
```
# use bifrost::*;
# struct Context {}
# let mut context = Context {};
# let dispatcher = Dispatcher::new(&mut context);
# let relay = dispatcher.create_relay();
let relay_clone = relay.clone();
```

**/
pub struct Relay<Context: Send> {
    event_sender: EventSender<Context>,
    command_sender: Sender<Command<Context>>,
}

impl<Context: Send> Clone for Relay<Context> {
    fn clone(&self) -> Self {
        Relay::new(self.event_sender.clone(),
                   self.command_sender.clone())
    }
}

impl<Context: Send> Relay<Context> {

    #[doc(hidden)]
    pub fn new(
        event_sender: EventSender<Context>,
        command_sender: Sender<Command<Context>>,
    ) -> Relay<Context> {
        Relay {
            event_sender,
            command_sender,
        }
    }

    pub fn send<E: Event<Context> + Send + 'static>(&self, event: E) {
        if let Err(e) = self.event_sender.send(Box::new(event)) {
            println!("Error sending event : {}", e);
        }
    }

    pub fn schedule<E: Event<Context> + Send + 'static>(&self, event: E, delay: Duration) {
        if let Err(e) = self.command_sender.send(Command::Schedule(
            Box::new(event),
            delay,
        )) {
            println!("Error scheduling event : {}", e);
        }
    }

    pub fn stop(&self) {
        if let Err(e) = self.command_sender.send(
            Command::Stop
        ) {
            println!("Error sending command : {}", e);
        }
    }
}