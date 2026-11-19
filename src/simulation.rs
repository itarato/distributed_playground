use std::collections::HashMap;

use uuid::Uuid;

use crate::{business::BusinessCore, channel::Channel, event::Event, node::Node};

#[derive(Default)]
pub struct Simulation {
    ticker: u64,
    nodes: HashMap<Uuid, Node>,
    channels: HashMap<Uuid, Channel>,
    edges: HashMap<(Uuid /* source node */, Uuid /* target node */), Uuid /* channel */>,
}

impl Simulation {
    pub fn add_node(&mut self, core: Box<dyn BusinessCore>) -> Uuid {
        let node = Node::new(core);
        let id = node.id();
        self.nodes.insert(node.id(), node);
        id
    }

    pub fn connect(&mut self, lhs_node_id: Uuid, rhs_node_id: Uuid) {
        let ch = Channel::new(lhs_node_id, rhs_node_id);

        self.edges.insert((lhs_node_id, rhs_node_id), ch.id());
        self.channels.insert(ch.id(), ch);
    }

    pub fn run(&mut self, limit: u64) {
        while self.ticker < limit {
            // Business cycles.
            for (_, node) in self.nodes.iter_mut() {
                for intent in node.tick(self.ticker) {
                    let ch_id = match self.edges.get(&(node.id(), intent.target)) {
                        Some(ch_id) => ch_id,
                        None => {
                            eprint!(
                                "Error: no comms declared between node {} and {}.",
                                node.id(),
                                intent.target
                            );
                            continue;
                        }
                    };

                    let event = Event::new(node.id(), intent.target, self.ticker, intent.payload);
                    self.channels
                        .get_mut(ch_id)
                        .expect("Missing channel")
                        .push_event(event);
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
