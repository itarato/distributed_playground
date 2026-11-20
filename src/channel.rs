use std::collections::VecDeque;

use uuid::Uuid;

use crate::event::Event;

struct ChannelState {
    out_of_order: f32,
    delay: f32,
    dead: bool,
}

impl Default for ChannelState {
    fn default() -> Self {
        Self {
            out_of_order: 0.0,
            delay: 0.0,
            dead: false,
        }
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
            state: ChannelState::default(),
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
}
