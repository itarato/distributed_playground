use std::collections::HashMap;

use uuid::Uuid;

use crate::{
    business::{BusinessCore, BusinessResult},
    channel::Channel,
    event::Event,
    node::Node,
};

#[derive(Default)]
pub struct Simulation<Payload> {
    ticker: u64,
    nodes: HashMap<Uuid, Node<Payload>>,
    nodes_by_name: HashMap<String, Uuid>,
    channels: HashMap<Uuid, Channel<Payload>>,
    edges: HashMap<(Uuid /* source node */, Uuid /* target node */), Uuid /* channel */>,
}

impl<Payload> Simulation<Payload> {
    pub fn add_node(&mut self, core: Box<dyn BusinessCore<PayloadType = Payload>>) -> Uuid {
        let node = Node::new(core);
        let id = node.id();

        self.nodes_by_name.insert(node.name(), id);
        self.nodes.insert(node.id(), node);

        id
    }

    pub fn connect(&mut self, lhs_node_id: Uuid, rhs_node_id: Uuid) {
        let ch = Channel::new(lhs_node_id, rhs_node_id);

        self.edges.insert((lhs_node_id, rhs_node_id), ch.id());
        self.channels.insert(ch.id(), ch);
    }

    pub fn run(&mut self, limit: u64) {
        'outer: while self.ticker < limit {
            // Business cycles.
            for (_, node) in self.nodes.iter_mut() {
                match node.tick(self.ticker) {
                    BusinessResult::Events(business_events) => {
                        for business_event in business_events {
                            let target_node_id = if let Some(target_node_id) =
                                self.nodes_by_name.get(&business_event.target)
                            {
                                target_node_id
                            } else {
                                eprintln!("Error: node not found: {}", business_event.target);
                                continue;
                            };

                            let ch_id = match self.edges.get(&(node.id(), *target_node_id)) {
                                Some(ch_id) => ch_id,
                                None => {
                                    eprint!(
                                        "Error: no comms declared between node {} and {}.",
                                        node.id(),
                                        target_node_id
                                    );
                                    continue;
                                }
                            };

                            let event =
                                Event::new(node.id(), *target_node_id, self.ticker, business_event);
                            self.channels
                                .get_mut(ch_id)
                                .expect("Missing channel")
                                .push_event(event);
                        }
                    }
                    BusinessResult::Halt => {
                        break 'outer;
                    }
                }
            }

            // Sending outgoing messages.
            for (_, channel) in self.channels.iter_mut() {
                for event in channel.consume_ready() {
                    self.nodes
                        .get_mut(&event.target)
                        .expect("Missing node")
                        .push_event(event);
                }
            }

            self.ticker += 1;
        }
    }
}
