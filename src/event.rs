use relay::Relay;

pub trait Event<Context: Send> {
    fn process(self: Box<Self>, relay: &Relay<Context>, ctx: &mut Context);
}
