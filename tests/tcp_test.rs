use std::collections::HashMap;

use distributed_playground::business::{BusinessCore, BusinessEvent, BusinessResult};

struct TcpMessage {
    source: String,
    seq_num: u32,
    ack_num: u32,
    ack: bool,
    syn: bool,
    data: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum ConnectionState {
    Closed,
    SynSent,
    SynReceived,
    Established,
}

enum ConnectionGoal {
    Send,
    ReactOnly,
}

struct Connection {
    state: ConnectionState,
    target: String,
    data_received: Vec<u8>,
    goal: ConnectionGoal,
}

impl Connection {
    fn new(target: String, goal: ConnectionGoal) -> Self {
        Self {
            state: ConnectionState::Closed,
            target,
            data_received: vec![],
            goal,
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
    fn new() -> Self {
        Self {
            state: SocketState::Listening,
            listening: false,
        }
    }
}

struct ServerCore {
    name: String,
    sockets: HashMap<String, Socket>,
    connections: HashMap<String, Connection>,
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
                assert!(self.sockets.get(&self.name).unwrap().listening);

                self.connections.insert(
                    target.clone(),
                    Connection::new(target.clone(), ConnectionGoal::Send),
                );
            }
        }

        let mut out_events = vec![];

        for (host, conn) in &self.connections {}

        BusinessResult::Events(out_events)
    }
}

impl ServerCore {
    fn new(name: String, is_listening: bool, send_target: Option<String>) -> Self {
        let sockets = if is_listening && send_target.is_some() {
            HashMap::from([(name.clone(), Socket::new())])
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
fn test_tcp_handshake() {}
