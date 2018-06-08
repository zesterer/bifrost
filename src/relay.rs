use dispatcher::Command;
use event::Event;
use event::EventSender;
use std::sync::mpsc::Sender;
use std::time::Duration;

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