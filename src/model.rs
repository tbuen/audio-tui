use std::sync::mpsc::Receiver;

use backend::{Backend, ChangeDirectory, Event};
use ratatui::widgets::ListState;

pub struct Model {
    backend: Backend,
    receiver: Receiver<Event>,
    connected: bool,
    current: String,
    files: Vec<String>,
    list_state: ListState,
    exit: bool,
}

impl Model {
    pub fn new() -> Self {
        let backend = Backend::new();
        let receiver = backend.receiver().unwrap();
        Model {
            backend,
            receiver,
            connected: Default::default(),
            current: String::new(),
            files: Vec::new(),
            list_state: ListState::default(),
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

    pub fn refresh_file_list(&mut self) {
        if let Ok(mut c) = self.backend.current_directory() {
            self.current = c.pop().unwrap();
        }
        if let Ok(c) = self.backend.directory_content() {
            self.files = c.dirs;
        }
        if self.files.is_empty() {
            self.list_state.select(None);
        } else {
            self.list_state.select_first();
        }
    }

    pub fn sync_files(&self) {
        self.backend.sync_files();
    }

    pub fn select_next(&mut self) {
        if let Some(cur) = self.list_state.selected()
            && cur + 1 < self.files.len()
        {
            self.list_state.select_next();
        }
    }

    pub fn select_prev(&mut self) {
        if self.list_state.selected().is_some() {
            self.list_state.select_previous();
        }
    }

    pub fn enter_dir(&mut self) {
        if let Some(cur) = self.list_state.selected()
            && let Some(dir) = self.files.get(cur)
        {
            self.backend
                .change_directory(ChangeDirectory::ToChild(dir))
                .unwrap();
            self.refresh_file_list();
        }
    }

    pub fn leave_dir(&mut self) {
        if !self.current().is_empty() {
            self.backend
                .change_directory(ChangeDirectory::ToParent)
                .unwrap();
            self.refresh_file_list();
        }
    }

    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn files(&self) -> Vec<&str> {
        self.files.iter().map(AsRef::as_ref).collect()
    }

    pub fn list_state(&self) -> &ListState {
        &self.list_state
    }
}
