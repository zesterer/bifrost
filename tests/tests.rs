#![feature(duration_extras)]

extern crate bifrost;

use bifrost::*;
use std::thread;
use std::time::Duration;
use std::time::Instant;

const EVENT_COUNT: i64 = 10000000;

// This is the "global context" the dispatcher gives to event handlers
struct TestContext {
    counter: i64
}


// This is an event
struct IncrementEvent {
    val: i64
}

// This is the handler of an event
impl Event<TestContext> for IncrementEvent {
    fn process(&self, _: &Relay<TestContext>, ctx: &mut TestContext) {
        ctx.counter += self.val;
    }
}


#[test]
fn simple_test() {
    let mut ctx = TestContext { counter: 0 };
    let timer = Instant::now();

    {
        // The dispatcher is running the event loop
        let mut dispatcher = Dispatcher::new(&mut ctx);

        // Relay are used to interact with the dispatcher
        // They can be cloned and sent to other threads safely
        let relay = dispatcher.create_relay();


        // Using the relay to send event to the dispatcher
        relay.send(IncrementEvent { val: 1 });


        // It also works from other threads
        let cloned_relay = relay.clone();
        thread::spawn(move || {
            for _ in 1..EVENT_COUNT {
                cloned_relay.send(IncrementEvent { val: 1 })
            }
        }).join().ok();

        // Make the dispatcher stop after having processed all the event sent above
        relay.send(
            event(|relay, _| {
                relay.schedule(
                    event(|relay, _| relay.stop()),
                    Duration::from_millis(1000)
                );
            })
        );

        // Let's run the event loop
        dispatcher.run();
    }

    println!("Context counter : {}", ctx.counter);

    let elapsed = timer.elapsed();
    let total_time = elapsed.as_secs() * 1000 + elapsed.subsec_millis() as u64;
    let events_per_sec = 1000f32 * (EVENT_COUNT as f32) / total_time as f32;
    println!("{} events dispatched on {} ms", EVENT_COUNT, total_time);
    println!("Average performance : {} events/second", events_per_sec);
    assert_eq!(ctx.counter, EVENT_COUNT);
}