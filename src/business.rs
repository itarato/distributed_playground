use crate::event::Event;

pub trait BusinessCore {
    fn tick(&mut self, tick: u64, events: Vec<Event>) -> Vec<Event>;
}
