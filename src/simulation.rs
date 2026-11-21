use std::collections::HashMap;

use log::{error, info};
use uuid::Uuid;

use crate::{
    business::{BusinessCore, BusinessResult},
    channel::{Channel, ChannelState},
    event::Event,
    node::{Node, NodeState},
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
    pub fn create_node(&mut self, core: Box<dyn BusinessCore<PayloadType = Payload>>) -> Uuid {
        let node = Node::new(core);
        let id = node.id();

        info!("Adding node <{}> as <{}>", node.name(), id);

        self.nodes_by_name.insert(node.name(), id);
        self.nodes.insert(id, node);

        id
    }

    pub fn connect(&mut self, lhs_node_id: Uuid, rhs_node_id: Uuid) -> Uuid {
        let ch = Channel::new(lhs_node_id, rhs_node_id);
        let id = ch.id();

        info!("Establish connection from <{lhs_node_id}> to <{rhs_node_id}>");

        self.edges.insert((lhs_node_id, rhs_node_id), ch.id());
        self.channels.insert(id, ch);

        id
    }

    pub fn run(&mut self, limit: u64) {
        info!("Simulation has started (limit: {} steps)", limit);

        let ticker_end = self.ticker + limit;

        'outer: while self.ticker < ticker_end {
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
                                error!("Node not found: {}", business_event.target);
                                continue;
                            };

                            let ch_id = match self.edges.get(&(node.id(), *target_node_id)) {
                                Some(ch_id) => ch_id,
                                None => {
                                    error!(
                                        "No channel declared between node {} and {}.",
                                        node.id(),
                                        target_node_id
                                    );
                                    continue;
                                }
                            };

                            let event =
                                Event::new(node.id(), *target_node_id, self.ticker, business_event);

                            match self.channels.get_mut(ch_id) {
                                Some(channel) => channel.push_event(event),
                                None => error!("Node not found"),
                            }
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
                    match self.nodes.get_mut(&event.target) {
                        Some(node) => node.push_event(event),
                        None => error!("Missing node"),
                    }
                }
            }

            self.ticker += 1;
        }
    }

    pub fn node_state_mut(&mut self, id: &Uuid) -> Option<&mut NodeState> {
        self.nodes.get_mut(id).map(|node| node.state_mut())
    }

    pub fn channel_state_mut(&mut self, id: &Uuid) -> Option<&mut ChannelState> {
        self.channels.get_mut(id).map(|channel| channel.state_mut())
    }
}
