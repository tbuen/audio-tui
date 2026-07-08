use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::model::Model;
use crate::update::Message;

pub fn handle_event(_model: &Model) -> Message {
    let mut msg = None;
    while msg.is_none() {
        msg = match event::read().unwrap() {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => handle_key(key_event),
            _ => None,
        };
    }
    msg.unwrap()
}

fn handle_key(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Left => Some(Message::Decrement),
        KeyCode::Right => Some(Message::Increment),
        _ => None,
    }
}
