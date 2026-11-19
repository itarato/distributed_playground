use uuid::Uuid;

use crate::{business::BusinessCore, event::Event};

#[derive(Default)]
struct NodeState {
    dead: bool,
}

pub struct Node {
    id: Uuid,
    state: NodeState,
    core: Box<dyn BusinessCore>,
    incoming_events: Vec<Event>,
}

impl Node {
    pub fn new(core: Box<dyn BusinessCore>) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: NodeState::default(),
            core,
            incoming_events: vec![],
        }
    }

    pub(crate) fn push_event(&mut self, event: Event) {
        self.incoming_events.push(event);
    }

    pub(crate) fn tick(&mut self, tick: u64) -> Vec<Event> {
        self.core
            .tick(tick, self.incoming_events.drain(..).collect())
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }
}
