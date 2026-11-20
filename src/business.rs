pub struct BusinessEvent<Payload> {
    pub payload: Payload,
    pub source: String,
    pub target: String,
}

impl<Payload> BusinessEvent<Payload> {
    pub fn new(payload: Payload, source: String, target: String) -> Self {
        Self {
            payload,
            source,
            target,
        }
    }
}

pub enum BusinessResult<Payload> {
    Events(Vec<BusinessEvent<Payload>>),
    Halt,
}

pub trait BusinessCore {
    type PayloadType;

    fn name(&self) -> String;
    fn tick(
        &mut self,
        tick: u64,
        events: Vec<BusinessEvent<Self::PayloadType>>,
    ) -> BusinessResult<Self::PayloadType>;
}
