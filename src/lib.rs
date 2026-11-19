mod business;
mod channel;
mod event;
mod node;
mod payload;
mod simulation;

#[cfg(test)]
mod test {
    use crate::{business::BusinessCore, event::Event, simulation::Simulation};

    #[derive(Default)]
    struct CustomCore {}

    impl BusinessCore for CustomCore {
        fn tick(&mut self, tick: u64, events: Vec<Event>) -> Vec<Event> {
            unimplemented!()
        }
    }

    #[test]
    fn smoke_test() {
        let core1 = CustomCore::default();
        let core2 = CustomCore::default();

        let mut sim = Simulation::default();

        let node1_id = sim.add_node(Box::new(core1));
        let node2_id = sim.add_node(Box::new(core2));

        sim.connect(node1_id, node2_id);
    }
}
