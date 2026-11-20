use uuid::Uuid;

use crate::{
    business::{BusinessCore, BusinessResult},
    event::Event,
};

#[derive(Default)]
struct NodeState {
    dead: bool,
}

pub struct Node<Payload> {
    id: Uuid,
    state: NodeState,
    core: Box<dyn BusinessCore<PayloadType = Payload>>,
    incoming_events: Vec<Event<Payload>>,
}

impl<Payload> Node<Payload> {
    pub fn new(core: Box<dyn BusinessCore<PayloadType = Payload>>) -> Self {
        Self {
            id: Uuid::new_v4(),
            state: NodeState::default(),
            core,
            incoming_events: vec![],
        }
    }

    pub(crate) fn push_event(&mut self, event: Event<Payload>) {
        self.incoming_events.push(event);
    }

    pub(crate) fn tick(&mut self, tick: u64) -> BusinessResult<Payload> {
        self.core.tick(
            tick,
            self.incoming_events
                .drain(..)
                .map(|e| e.business_event)
                .collect(),
        )
    }

    pub(crate) fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> String {
        self.core.name()
    }
}
