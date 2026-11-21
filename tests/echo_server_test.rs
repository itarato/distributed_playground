use std::{cell::RefCell, rc::Rc};

use distributed_playground::{
    business::{BusinessCore, BusinessEvent, BusinessResult},
    simulation::Simulation,
};
use uuid::Uuid;

struct ClientCore {
    history: Rc<RefCell<Vec<String>>>,
    counter: u32,
}

impl ClientCore {
    fn new(history: Rc<RefCell<Vec<String>>>) -> Self {
        Self {
            history,
            counter: 0,
        }
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

        self.counter += 1;

        BusinessResult::Events(vec![BusinessEvent::new(
            format!("Hello, {} world!", self.counter - 1),
            self.name(),
            "server".to_string(),
        )])
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

    setup.sim.run(4);
    assert_eq!(2, setup.client_history.borrow().len());

    setup.sim.run(1);
    assert_eq!(3, setup.client_history.borrow().len());

    setup.sim.run(7);
    assert_eq!(10, setup.client_history.borrow().len());

    for i in 0..10 {
        assert_eq!(
            format!("Hello, {} world!", i),
            setup.client_history.borrow()[i],
        );
    }
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

#[test]
fn test_server_node_dead() {
    let mut setup = EchoServer::new();
    setup
        .sim
        .node_state_mut(&setup.server_node_id)
        .unwrap()
        .set_alive(false);

    setup.sim.run(10);

    assert_eq!(0, setup.client_history.borrow().len());
}

#[test]
fn test_client_node_temporarily_dead() {
    let mut setup = EchoServer::new();

    setup
        .sim
        .node_state_mut(&setup.client_node_id)
        .unwrap()
        .set_alive(false);

    setup.sim.run(10);
    assert_eq!(0, setup.client_history.borrow().len());

    setup
        .sim
        .node_state_mut(&setup.client_node_id)
        .unwrap()
        .set_alive(true);

    setup.sim.run(10);
    assert_eq!(8, setup.client_history.borrow().len());
}

#[test]
fn test_channel_out_of_order() {
    let mut setup = EchoServer::new();

    setup
        .sim
        .channel_state_mut(&setup.client_to_server_channel_id);

    setup.sim.run(102);
    assert_eq!(100, setup.client_history.borrow().len());

    for i in 0..100 {
        assert_eq!(
            format!("Hello, {} world!", i),
            setup.client_history.borrow()[i],
        );
    }
}
