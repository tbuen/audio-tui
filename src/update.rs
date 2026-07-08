use crate::model::Model;

pub enum Message {
    Increment,
    Decrement,
    Quit,
}

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::Increment => model.increment_counter(),
        Message::Decrement => model.decrement_counter(),
        Message::Quit => model.exit(),
    }
}
