use std::collections::VecDeque;

use uuid::Uuid;

use crate::event::Event;

pub struct ChannelState {
    out_of_order: usize,
    delay: f32,
    active: bool,
}

impl ChannelState {
    fn new() -> Self {
        Self {
            out_of_order: 0,
            delay: 0.0,
            active: true,
        }
    }

    pub fn set_out_of_order(&mut self, value: usize) {
        self.out_of_order = value;
    }
}

pub struct Channel<T> {
    id: Uuid,
    queue: VecDeque<Event<T>>,
    state: ChannelState,
    source: Uuid,
    target: Uuid,
}

impl<T> Channel<T> {
    pub(crate) fn new(source: Uuid, target: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            queue: VecDeque::default(),
            state: ChannelState::new(),
            source,
            target,
        }
    }

    pub(crate) fn consume_ready(&mut self) -> Vec<Event<T>> {
        self.queue.drain(..).collect()
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn push_event(&mut self, event: Event<T>) {
        self.queue.push_back(event);
    }

    pub(crate) fn state_mut(&mut self) -> &mut ChannelState {
        &mut self.state
    }
}
