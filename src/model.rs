#[derive(Default)]
pub struct Model {
    counter: u8,
    wants_exit: bool,
}

impl Model {
    pub fn wants_exit(&self) -> bool {
        self.wants_exit
    }

    pub fn exit(&mut self) {
        self.wants_exit = true;
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
