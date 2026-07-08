use crate::model::Model;

pub enum Message {
    ConnectionState(bool),
    Increment,
    Decrement,
    Resize,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Increment => model.increment_counter(),
        Message::Decrement => model.decrement_counter(),
        Message::ConnectionState(value) => model.set_connected(value),
        Message::Quit => model.set_exit(),
        Message::Resize => {}
    }
}
