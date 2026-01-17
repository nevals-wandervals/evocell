pub trait App: EventHandler {
    fn new() -> Self;
    fn init(self) -> Self;
    fn update(&mut self);
    fn render(&mut self);
    fn run(mut self)
    where
        Self: Sized,
    {
        loop {
            if self.event_handler() {
                break;
            }
            self.update();
            self.render();
        }
    }
}

pub trait EventHandler {
    /// true - stop main loop
    fn event_handler(&mut self) -> bool;
}
