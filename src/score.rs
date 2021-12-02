use crate::event::Event;

pub struct Score {}

impl Score {
    pub fn new() -> Self {
        Self {}
    }

    pub fn process_event(&mut self, event: &Event) {
        println!("EVENT: {:?}", event);
    }
}
