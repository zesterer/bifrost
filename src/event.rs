use relay::Relay;

pub trait Event<Context: Send> {
    fn process(&self, relay: &Relay<Context>, ctx: &mut Context);
}
