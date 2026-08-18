use std::time::Duration;

use backend::{Event, FileSync, TagSync};
use crossterm::event;
use crossterm::event::{KeyCode, KeyEvent, KeyEventKind};

use crate::model::Model;
use crate::update::Message;

pub fn handle_event(model: &Model) -> Message {
    let mut msg = None;
    while msg.is_none() {
        if let Ok(true) = event::poll(Duration::from_millis(10)) {
            match event::read().unwrap() {
                event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    msg = handle_key(key_event);
                }
                event::Event::Resize(_, _) => msg = Some(Message::Resize),
                _ => {}
            }
        }
        if msg.is_none()
            && let Ok(evt) = model.receiver().recv_timeout(Duration::from_millis(10))
        {
            msg = handle_backend(evt);
        }
        if msg.is_none() && model.last_tick().elapsed() >= Duration::from_millis(100) {
            msg = Some(Message::Tick);
        }
    }
    msg.unwrap()
}

fn handle_key(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Char('q') => Some(Message::Quit),
        KeyCode::Char('s') => Some(Message::SyncFiles),
        KeyCode::Char('S') => Some(Message::SyncTags),
        KeyCode::Char('j') | KeyCode::Down => Some(Message::NextFile),
        KeyCode::Char('k') | KeyCode::Up => Some(Message::PrevFile),
        KeyCode::Char('l') | KeyCode::Right => Some(Message::EnterDir),
        KeyCode::Char('h') | KeyCode::Left => Some(Message::LeaveDir),
        _ => None,
    }
}

fn handle_backend(event: Event) -> Option<Message> {
    match event {
        Event::Connected => Some(Message::ConnectionState(true)),
        Event::Disconnected => Some(Message::ConnectionState(false)),
        // TODO handle errors as well...
        Event::FileSync(FileSync::Started) => Some(Message::FileSyncStarted),
        Event::FileSync(FileSync::Completed) => Some(Message::FileSyncCompleted),
        Event::FileSync(FileSync::Aborted) => Some(Message::FileSyncAborted),
        Event::TagSync(TagSync::Started) => Some(Message::TagSyncStarted),
        Event::TagSync(TagSync::Completed) => Some(Message::TagSyncCompleted),
        Event::TagSync(TagSync::Aborted) => Some(Message::TagSyncAborted),
        Event::TagSync(TagSync::Step(n, t)) => Some(Message::TagSyncStep(n, t)),
        _ => None,
    }
}
