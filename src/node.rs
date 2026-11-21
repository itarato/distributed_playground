use uuid::Uuid;

use crate::{
    business::{BusinessCore, BusinessResult},
    event::Event,
};

pub struct NodeState {
    alive: bool,
}

impl NodeState {
    pub(crate) fn new() -> Self {
        Self { alive: true }
    }

    pub fn set_alive(&mut self, value: bool) {
        self.alive = value;
    }
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
            state: NodeState::new(),
            core,
            incoming_events: vec![],
        }
    }

    pub(crate) fn push_event(&mut self, event: Event<Payload>) {
        self.incoming_events.push(event);
    }

    pub(crate) fn tick(&mut self, tick: u64) -> BusinessResult<Payload> {
        let events = self
            .incoming_events
            .drain(..)
            .map(|e| e.business_event)
            .collect();

        if self.state.alive {
            self.core.tick(tick, events)
        } else {
            BusinessResult::Events(vec![])
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub(crate) fn name(&self) -> String {
        self.core.name()
    }

    pub fn state_mut(&mut self) -> &mut NodeState {
        &mut self.state
    }
}
