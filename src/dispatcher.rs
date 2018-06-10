use channel::Channel;
use command::Command;
use relay::Relay;
use scheduler::DEFAULT_SCHEDULER_PRECISION;
use scheduler::ScheduledEvent;
use std::collections::LinkedList;
use std::sync::mpsc::TryRecvError;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use types::BoxedEvent;


pub struct Dispatcher<'a, Context: 'a + Send> {
    event_channel: Channel<BoxedEvent<Context>>,
    command_channel: Channel<Command<Context>>,
    relay: Relay<Context>,
    context: &'a mut Context,

    running: bool,

    // Scheduling
    scheduled_events: LinkedList<ScheduledEvent<Context>>,
    scheduling_precision: Duration,
    scheduling_timer: Instant,
}

impl<'a, Context: 'a + Send> Dispatcher<'a, Context> {
    pub fn new(context: &mut Context) -> Dispatcher<Context> {
        let event_channel = Channel::<BoxedEvent<Context>>::new();
        let command_channel = Channel::<Command<Context>>::new();

        let relay = Relay::new(
            event_channel.get_sender().clone(),
            command_channel.get_sender().clone(),
        );

        Dispatcher {
            event_channel,
            command_channel,
            relay,
            context,
            running: false,
            scheduled_events: LinkedList::new(),
            scheduling_precision: Duration::from_millis(DEFAULT_SCHEDULER_PRECISION),
            scheduling_timer: Instant::now(),
        }
    }

    pub fn create_relay(&self) -> Relay<Context> {
        self.relay.clone()
    }

    pub fn run(&mut self) {
        self.running = true;

        while self.running {
            let elapsed = self.scheduling_timer.elapsed();

            if elapsed < self.scheduling_precision {
                thread::sleep(self.scheduling_precision - elapsed);
                continue;
            } else {
                self.scheduling_timer = Instant::now();
            }

            while self.handle_commands() && self.scheduling_timer.elapsed() < self.scheduling_precision {}
            self.handle_scheduled_events(elapsed);
            while self.handle_events() && self.scheduling_timer.elapsed() < self.scheduling_precision {}
        }
    }

    #[inline]
    fn handle_commands(&mut self) -> bool {
        match self.command_channel.get_receiver().try_recv() {
            Ok(command) => {
                self.process_command(command);
                return true;
            }
            Err(err) => {
                match err {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        self.running = false;
                        println!("Channel disconnected")
                    }
                };
                return false;
            }
        }
    }

    #[inline]
    fn process_command(&mut self, command: Command<Context>) {
        match command {
            Command::Stop => self.running = false,
            Command::Schedule(event, delay) => self.scheduled_events.push_back(ScheduledEvent::new(event, delay)),
        }
    }

    #[inline]
    fn handle_scheduled_events(&mut self, elapsed: Duration) {
        let sender = self.event_channel.get_sender();

        self.scheduled_events.drain_filter(
            |event| -> bool {
                event.advance(elapsed);
                event.is_ready()
            }
        ).for_each(
            |event|
                sender.send(event.get_event()).unwrap()
        );
    }

    #[inline]
    fn handle_events(&mut self) -> bool {
        match self.event_channel.get_receiver().try_recv() {
            Ok(event) => {
                event.process(&mut self.relay, &mut self.context);
                return true;
            }
            Err(err) => {
                match err {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        self.running = false;
                        println!("Channel disconnected");
                    }
                };
                return false;
            }
        }
    }
}