use std::sync::mpsc::Receiver;

use backend::{Backend, Event};

pub struct Model {
    _backend: Backend,
    receiver: Receiver<Event>,
    connected: bool,
    counter: u8,
    exit: bool,
}

impl Model {
    pub fn new() -> Self {
        let backend = Backend::new();
        let receiver = backend.receiver().unwrap();
        Model {
            _backend: backend,
            receiver,
            connected: Default::default(),
            counter: Default::default(),
            exit: Default::default(),
        }
    }

    pub fn receiver(&self) -> &Receiver<Event> {
        &self.receiver
    }

    pub fn exit(&self) -> bool {
        self.exit
    }

    pub fn set_exit(&mut self) {
        self.exit = true;
    }

    pub fn connected(&self) -> bool {
        self.connected
    }

    pub fn set_connected(&mut self, val: bool) {
        self.connected = val;
    }

    pub fn counter(&self) -> u8 {
        self.counter
    }

    pub fn increment_counter(&mut self) {
        self.counter += 1;
    }

    pub fn decrement_counter(&mut self) {
        self.counter -= 1;
    }
}
