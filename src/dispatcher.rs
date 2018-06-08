use channel::Channel;
use event::BoxedEvent;
use relay::Relay;
use std::sync::mpsc::TryRecvError;
use std::time::Duration;
use std::collections::LinkedList;
use std::time::Instant;
use std::thread;

pub type EventChannel<Context> = Channel<BoxedEvent<Context>>;

pub enum Command<Context: Send> {
    Stop,
    Schedule(BoxedEvent<Context>, Duration),
}

const DEFAULT_SCHEDULER_PRECISION: u64 = 50; // ms

pub struct ScheduledEvent<Context: Send> {
    event: BoxedEvent<Context>,
    delay: Duration,
}

impl<Context: Send> ScheduledEvent<Context> {
    pub fn new(event: BoxedEvent<Context>, delay: Duration) -> ScheduledEvent<Context> {
        ScheduledEvent {
            event,
            delay,
        }
    }

    pub fn advance(&mut self, duration: Duration) {
        if self.delay > duration {
            self.delay -= duration;
        } else {
            self.delay = Duration::from_millis(0);
        }

    }

    pub fn is_ready(&self) -> bool {
        self.delay == Duration::from_millis(0)
    }
}


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

impl<'a, Context: 'a + Send + Sync> Dispatcher<'a, Context> {
    pub fn new(context: &mut Context) -> Dispatcher<Context> {

        let event_channel = Channel::<BoxedEvent<Context>>::new();
        let command_channel = Channel::<Command<Context>>::new();

        let relay = Relay::new(
            event_channel.get_sender().clone(),
            command_channel.get_sender().clone()
        );

        Dispatcher {
            event_channel,
            command_channel,
            relay,
            context,
            running: false,
            scheduled_events: LinkedList::new(),
            scheduling_precision: Duration::from_millis(DEFAULT_SCHEDULER_PRECISION),
            scheduling_timer: Instant::now()
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
                return true
            },
            Err(err) => {
                match err {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        self.running = false;
                        println!("Channel disconnected")
                    }
                };
                return false
            },
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
                sender.send(event.event).unwrap()
        );
    }

    #[inline]
    fn handle_events(&mut self) -> bool {
        match self.event_channel.get_receiver().try_recv() {
            Ok(event) => {
                event.process(&mut self.relay, &mut self.context);
                return true
            },
            Err(err) => {
                match err {
                    TryRecvError::Empty => (),
                    TryRecvError::Disconnected => {
                        self.running = false;
                        println!("Channel disconnected");
                    }
                };
                return false
            },
        }
    }
}