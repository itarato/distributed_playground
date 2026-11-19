use uuid::Uuid;

use crate::payload::Payload;

pub struct EventIntent {
    payload: Payload,
    target: Uuid,
}

pub struct Event {
    pub source: Uuid,
    pub target: Uuid,
    pub t_create: u64,
    pub payload: Payload,
}

impl Event {
    pub(crate) fn new(source: Uuid, target: Uuid, t_create: u64, payload: Payload) -> Self {
        Self {
            source,
            target,
            t_create,
            payload,
        }
    }
}
