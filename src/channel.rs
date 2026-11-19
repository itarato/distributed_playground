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

pub struct Channel {
    id: Uuid,
    queue: VecDeque<Event>,
    state: ChannelState,
    source: Uuid,
    target: Uuid,
}

impl Channel {
    pub(crate) fn new(source: Uuid, target: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            queue: VecDeque::default(),
            state: ChannelState::default(),
            source,
            target,
        }
    }

    pub(crate) fn consume_ready(&mut self) -> Vec<Event> {
        unimplemented!()
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        self.queue.push_back(event);
    }
}
