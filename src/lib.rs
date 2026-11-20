mod business;
mod channel;
mod event;
mod node;
mod simulation;

#[cfg(test)]
mod test {
    use crate::{
        business::{BusinessCore, BusinessEvent, BusinessResult},
        simulation::Simulation,
    };

    struct ClientCore {}

    impl BusinessCore for ClientCore {
        type PayloadType = String;

        fn name(&self) -> String {
            String::from("client")
        }

        fn tick(
            &mut self,
            tick: u64,
            events: Vec<BusinessEvent<String>>,
        ) -> BusinessResult<String> {
            let events = if tick < 3 {
                vec![BusinessEvent::new(
                    "Hello, world!".to_string(),
                    self.name(),
                    "server".to_string(),
                )]
            } else {
                vec![]
            };

            BusinessResult::Events(events)
        }
    }

    struct ServerCore {}

    impl BusinessCore for ServerCore {
        type PayloadType = String;

        fn name(&self) -> String {
            String::from("server")
        }

        fn tick(
            &mut self,
            tick: u64,
            events: Vec<BusinessEvent<String>>,
        ) -> BusinessResult<String> {
            BusinessResult::Events(
                events
                    .iter()
                    .map(|event| {
                        BusinessEvent::new(
                            event.payload.clone(),
                            "server".to_string(),
                            event.source.clone(),
                        )
                    })
                    .collect(),
            )
        }
    }

    #[test]
    fn smoke_test() {
        let core1 = ClientCore {};
        let core2 = ServerCore {};

        let mut sim = Simulation::default();

        let node1_id = sim.add_node(Box::new(core1));
        let node2_id = sim.add_node(Box::new(core2));

        sim.connect(node1_id, node2_id);

        sim.run(10);
    }
}
