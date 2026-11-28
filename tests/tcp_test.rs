use std::collections::HashMap;

use distributed_playground::{
    business::{BusinessCore, BusinessEvent, BusinessResult},
    simulation::Simulation,
};

#[derive(Default, Debug)]
struct TcpMessage {
    source: String,
    target: String,
    seq_num: u32,
    ack_num: u32,
    ack: bool,
    syn: bool,
    rst: bool,
    data: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
enum ConnectionState {
    Closed,
    Listening,
    SynSent,
    SynReceived,
    Established,
}

#[derive(Debug)]
enum ConnectionGoal {
    Send,
    ReactOnly,
}

#[derive(Debug)]
struct Connection {
    state: ConnectionState,
    source: String,
    target: String,
    data_received: Vec<u8>,
    goal: ConnectionGoal,
}

impl Connection {
    fn new(source: String, target: String, goal: ConnectionGoal, state: ConnectionState) -> Self {
        Self {
            state,
            source,
            target,
            data_received: vec![],
            goal,
        }
    }

    fn respond(&mut self, message: TcpMessage) -> Vec<TcpMessage> {
        if message.syn && !message.ack && self.state == ConnectionState::Listening {
            // Connection attempt: Stage 1: SYN
            self.state = ConnectionState::SynReceived;

            vec![TcpMessage {
                source: message.target.clone(),
                target: message.source.clone(),
                seq_num: 300,
                ack_num: message.seq_num + 1,
                ack: true,
                syn: true,
                rst: false,
                data: vec![],
            }]
        } else if message.syn && message.ack && self.state == ConnectionState::SynSent {
            // Connection attempt: Stage 2: SYN ACK
            self.state = ConnectionState::Established;

            vec![TcpMessage {
                source: message.target.clone(),
                target: message.source.clone(),
                seq_num: message.ack_num,
                ack_num: message.seq_num + 1,
                ack: true,
                syn: false,
                rst: false,
                data: vec![],
            }]
        } else if message.ack && !message.syn && self.state == ConnectionState::SynReceived {
            // Connection attempt: Stage 3: ACK
            self.state = ConnectionState::Established;

            vec![]
        } else {
            dbg!(message);
            dbg!(self);
            unimplemented!()
        }
    }

    fn tick(&mut self) -> Vec<TcpMessage> {
        match self.goal {
            ConnectionGoal::Send => match self.state {
                ConnectionState::Closed => {
                    // Connection attempt: Stage 1: SYN
                    self.state = ConnectionState::SynSent;

                    vec![TcpMessage {
                        source: self.source.clone(),
                        target: self.target.clone(),
                        seq_num: 100,
                        ack_num: 0,
                        ack: false,
                        syn: true,
                        rst: false,
                        data: vec![],
                    }]
                }
                ConnectionState::Established => {
                    // Noop
                    vec![]
                }
                ConnectionState::Listening => {
                    // Noop
                    vec![]
                }
                ConnectionState::SynReceived => {
                    // Noop
                    vec![]
                }
                ConnectionState::SynSent => {
                    // Noop
                    vec![]
                }
            },
            ConnectionGoal::ReactOnly => vec![],
        }
    }
}

enum SocketState {
    Closed,
    Listening,
}

struct Socket {
    state: SocketState,
    listening: bool,
}

impl Socket {
    fn new(listening: bool) -> Self {
        Self {
            state: SocketState::Listening,
            listening,
        }
    }
}

struct ServerCore {
    name: String,
    sockets: HashMap<String /* Source */, Socket>,
    connections: HashMap<(String /* Spource */, String /* Target */), Connection>,
    send_target: Option<String>,
}

impl BusinessCore for ServerCore {
    type PayloadType = TcpMessage;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn tick(
        &mut self,
        tick: u64,
        events: Vec<BusinessEvent<Self::PayloadType>>,
    ) -> distributed_playground::business::BusinessResult<Self::PayloadType> {
        if tick == 0 {
            if let Some(target) = self.send_target.as_ref() {
                self.sockets
                    .get_mut(&self.name)
                    .expect(format!("Socket {} not found", &self.name).as_str())
                    .listening = true;

                self.connections.insert(
                    (self.name.clone(), target.clone()),
                    Connection::new(
                        self.name().clone(),
                        target.clone(),
                        ConnectionGoal::Send,
                        ConnectionState::Closed,
                    ),
                );
            }
        }

        let mut out_events = vec![];

        for event in events {
            if let Some(connection) = self
                .connections
                .get_mut(&(event.payload.target.clone(), event.payload.source.clone()))
            {
                // Has an active connection.
                let response_tcp_messages = connection.respond(event.payload);
                out_events.append(
                    &mut response_tcp_messages
                        .into_iter()
                        .map(|tcp_msg| {
                            BusinessEvent::new(tcp_msg, self.name().clone(), event.target.clone())
                        })
                        .collect(),
                );
            } else {
                if let Some(socket) = self.sockets.get(&event.payload.target) {
                    // Has a socket on the target.
                    if socket.listening {
                        if event.payload.syn {
                            let connection = Connection::new(
                                self.name().clone(),
                                event.payload.source.clone(),
                                ConnectionGoal::ReactOnly,
                                ConnectionState::Listening,
                            );
                            let source_target_pair =
                                (self.name.clone(), event.payload.source.clone());
                            self.connections
                                .insert(source_target_pair.clone(), connection);

                            out_events.append(
                                &mut self
                                    .connections
                                    .get_mut(&source_target_pair)
                                    .unwrap()
                                    .respond(event.payload)
                                    .into_iter()
                                    .map(|tcp_msg| {
                                        BusinessEvent::new(
                                            tcp_msg,
                                            self.name().clone(),
                                            event.source.clone(),
                                        )
                                    })
                                    .collect(),
                            );
                        } else {
                            unimplemented!()
                        }
                    } else {
                        out_events.push(BusinessEvent::new(
                            TcpMessage {
                                source: self.name().clone(),
                                target: event.payload.source.clone(),
                                seq_num: 0,
                                ack_num: event.payload.seq_num + 1,
                                ack: true,
                                syn: false,
                                rst: true,
                                data: vec![],
                            },
                            self.name().clone(),
                            event.source.clone(),
                        ));
                    }
                } else {
                    // No socket.
                }
            }
        }

        // Generic connection ticks.
        for (_, connection) in self.connections.iter_mut() {
            let out_tcp_events = connection.tick();
            let mut out_business_events: Vec<_> = out_tcp_events
                .into_iter()
                .map(|payload| {
                    let source = payload.source.clone();
                    let target = payload.target.clone();
                    BusinessEvent::new(payload, source, target)
                })
                .collect();

            out_events.append(&mut out_business_events);
        }

        BusinessResult::Events(out_events)
    }
}

impl ServerCore {
    fn new(name: String, is_listening: bool, send_target: Option<String>) -> Self {
        let sockets = if is_listening || send_target.is_some() {
            HashMap::from([(name.clone(), Socket::new(is_listening))])
        } else {
            HashMap::new()
        };

        Self {
            name,
            connections: HashMap::new(),
            sockets,
            send_target,
        }
    }
}

#[test]
fn test_tcp_handshake() {
    let core_sender = ServerCore::new("sender".to_string(), false, Some("receiver".to_string()));
    let core_receiver = ServerCore::new("receiver".to_string(), true, None);

    let mut sim = Simulation::default();

    let sender_node_id = sim.create_node(Box::new(core_sender));
    let receiver_node_id = sim.create_node(Box::new(core_receiver));

    sim.connect(sender_node_id, receiver_node_id);
    sim.connect(receiver_node_id, sender_node_id);

    sim.run(10);
}
