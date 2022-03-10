pub enum Event {
    Paint,
    Resize,
}

pub trait Application {
    fn event_handler(event: Event);
}
