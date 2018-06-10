use std::time::Duration;
use types::BoxedEvent;

pub const DEFAULT_SCHEDULER_PRECISION: u64 = 50; // ms

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

    pub fn get_event(self) -> BoxedEvent<Context> { self.event }
}