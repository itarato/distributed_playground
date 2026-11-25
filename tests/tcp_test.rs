use distributed_playground::business::BusinessCore;

struct TcpMessage {
    source: String,
    seq_num: u32,
    ack_num: u32,
    ack: bool,
    syn: bool,
    data: Vec<u8>,
}

#[derive(PartialEq, Eq)]
enum ServerState {
    Closed,
    Listen,
    SynSent,
    SynReceived,
    Established,
}

enum Task {
    Nothing,
    Listen,
    SendMessage,
}

struct ServerCore {
    state: ServerState,
    name: String,
    task: Task,
}

impl BusinessCore for ServerCore {
    type PayloadType = TcpMessage;

    fn name(&self) -> String {
        self.name.clone()
    }

    fn tick(
        &mut self,
        tick: u64,
        events: Vec<distributed_playground::business::BusinessEvent<Self::PayloadType>>,
    ) -> distributed_playground::business::BusinessResult<Self::PayloadType> {
        if self.state == ServerState::Closed {
            match self.task {
                Task::SendMessage => {}
                Task::Listen => {
                    self.state = ServerState::Listen;
                }
                Task::Nothing => {}
            }
        }

        unimplemented!()
    }
}

impl ServerCore {
    fn new(name: String, task: Task) -> Self {
        Self {
            state: ServerState::Closed,
            name,
            task,
        }
    }
}

#[test]
fn test_tcp_handshake() {}
