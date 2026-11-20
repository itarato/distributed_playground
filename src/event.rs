use uuid::Uuid;

use crate::business::BusinessEvent;

pub struct Event<T> {
    pub source: Uuid,
    pub target: Uuid,
    pub t_create: u64,
    pub business_event: BusinessEvent<T>,
}

impl<T> Event<T> {
    pub(crate) fn new(
        source: Uuid,
        target: Uuid,
        t_create: u64,
        business_event: BusinessEvent<T>,
    ) -> Self {
        Self {
            source,
            target,
            t_create,
            business_event,
        }
    }
}
