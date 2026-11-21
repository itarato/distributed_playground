use crate::event::Event;
use std::collections::VecDeque;
use uuid::Uuid;

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
        if !self.state.active {
            return vec![];
        }

        self.queue.drain(..).collect()
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn push_event(&mut self, event: Event<T>) {
        if !self.state.active {
            return;
        }

        if self.state.out_of_order == 0 || self.queue.is_empty() {
            self.queue.push_back(event);
        } else {
            let random_pos = rand::random::<u32>() % self.queue.len() as u32;
            self.queue.insert(random_pos as usize, event);
        }
    }

    pub(crate) fn state_mut(&mut self) -> &mut ChannelState {
        &mut self.state
    }
}
