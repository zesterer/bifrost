use std::time::Duration;
use types::BoxedEvent;

pub enum Command<Context: Send> {
    Stop,
    Schedule(BoxedEvent<Context>, Duration),
}