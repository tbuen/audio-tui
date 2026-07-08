use std::time::Duration;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};

use crate::model::Model;
use crate::update::Message;

pub fn handle_event(model: &Model) -> Message {
    let mut msg = None;
    while msg.is_none() {
        if let Ok(true) = event::poll(Duration::from_millis(10)) {
            match event::read().unwrap() {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    msg = handle_key(key_event);
                }
                Event::Resize(_, _) => msg = Some(Message::Resize),
                _ => {}
            }
        }
        if msg.is_none()
            && let Ok(evt) = model.receiver().recv_timeout(Duration::from_millis(10))
        {
            msg = handle_backend(evt);
        }
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

fn handle_backend(event: backend::Event) -> Option<Message> {
    match event {
        backend::Event::Connected => Some(Message::ConnectionState(true)),
        backend::Event::Disconnected => Some(Message::ConnectionState(false)),
        _ => None,
    }
}
