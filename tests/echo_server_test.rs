use std::{cell::RefCell, rc::Rc};

use distributed_playground::{
    business::{BusinessCore, BusinessEvent, BusinessResult},
    simulation::Simulation,
};
use uuid::Uuid;

struct ClientCore {
    history: Rc<RefCell<Vec<String>>>,
}

impl ClientCore {
    fn new(history: Rc<RefCell<Vec<String>>>) -> Self {
        Self { history }
    }
}

impl BusinessCore for ClientCore {
    type PayloadType = String;

    fn name(&self) -> String {
        String::from("client")
    }

    fn tick(&mut self, tick: u64, events: Vec<BusinessEvent<String>>) -> BusinessResult<String> {
        for event in events {
            self.history.borrow_mut().push(event.payload);
        }

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

#[derive(Default)]
struct ServerCore {}

impl BusinessCore for ServerCore {
    type PayloadType = String;

    fn name(&self) -> String {
        String::from("server")
    }

    fn tick(&mut self, _tick: u64, events: Vec<BusinessEvent<String>>) -> BusinessResult<String> {
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

struct EchoServer {
    sim: Simulation<String>,
    client_node_id: Uuid,
    server_node_id: Uuid,
    client_to_server_channel_id: Uuid,
    server_to_client_channel_id: Uuid,
    client_history: Rc<RefCell<Vec<String>>>,
}

impl EchoServer {
    fn new() -> Self {
        let _ = env_logger::try_init();

        let client_history = Rc::new(RefCell::new(vec![]));

        let client = ClientCore::new(client_history.clone());
        let server = ServerCore::default();

        let mut sim = Simulation::default();

        let client_node_id = sim.create_node(Box::new(client));
        let server_node_id = sim.create_node(Box::new(server));

        let client_to_server_channel_id = sim.connect(client_node_id, server_node_id);
        let server_to_client_channel_id = sim.connect(server_node_id, client_node_id);

        Self {
            sim,
            client_node_id,
            server_node_id,
            client_history,
            client_to_server_channel_id,
            server_to_client_channel_id,
        }
    }
}

#[test]
fn test_happy_path() {
    let mut setup = EchoServer::new();

    setup.sim.run(10);

    assert_eq!(3, setup.client_history.borrow().len());
}

#[test]
fn test_client_node_dead() {
    let mut setup = EchoServer::new();
    setup
        .sim
        .node_state_mut(&setup.client_node_id)
        .unwrap()
        .set_alive(false);

    setup.sim.run(10);

    assert_eq!(0, setup.client_history.borrow().len());
}
