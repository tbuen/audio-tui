use std::time::Instant;
use std::{sync::mpsc::Receiver, time::Duration};

use backend::{Backend, ChangeDirection, Event, FileViewContent, TagViewContent};
use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use ratatui_comfy_toaster::{
    ToastBuilder, ToastEngine, ToastEngineBuilder, ToastPosition, ToastPreset,
    ToastProgressBarStyle, ToastType,
};
use throbber_widgets_tui::ThrobberState;

#[derive(Debug, Copy, Clone)]
pub enum Pane {
    Files,
    Tags,
}

pub struct Model {
    backend: Backend,
    receiver: Receiver<Event>,
    exit: bool,
    connected: bool,
    active_pane: Pane,
    fileview: View,
    tagview: View,
    last_tick: Instant,
    throbbing: bool,
    throbber_state: ThrobberState,
    gauge: Option<u16>,
    toast_engine: ToastEngine<()>,
}

#[derive(Debug, Default)]
pub struct View {
    current: String,
    content: Vec<String>,
    list_state: ListState,
}

impl Model {
    pub fn new() -> Self {
        let backend = Backend::new();
        let receiver = backend.receiver().unwrap();
        Model {
            backend,
            receiver,
            exit: false,
            connected: false,
            active_pane: Pane::Files,
            fileview: View::default(),
            tagview: View::default(),
            last_tick: Instant::now(),
            throbbing: false,
            throbber_state: ThrobberState::default(),
            gauge: None,
            toast_engine: ToastEngineBuilder::new(Rect::new(0, 0, 120, 40))
                .default_duration(Duration::from_secs(3))
                .default_progress_bar(true)
                .default_progress_bar_style(ToastProgressBarStyle::Minimal)
                .build(),
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

    // pane

    pub fn active_pane(&self) -> Pane {
        self.active_pane
    }

    pub fn set_active_pane(&mut self, val: Pane) {
        self.active_pane = val;
    }

    // file view

    pub fn sync_files(&self) {
        self.backend.sync_files();
    }

    pub fn fileview(&mut self) -> &mut View {
        &mut self.fileview
    }

    pub fn fileview_refresh(&mut self, select: Option<String>) {
        let current = self.backend.fileview_current().pop().unwrap_or_default();
        let content = match self.backend.fileview_content() {
            FileViewContent::Folders(folders) => folders,
            FileViewContent::Files { tracks, .. } => tracks,
        };
        self.fileview = View::new(current, content, select);
    }

    pub fn fileview_enter(&mut self) {
        if let Some(dir) = self.fileview.enter()
            && let FileViewContent::Folders(folders) = self.backend.fileview_content()
            && folders.iter().find(|f| *f == dir).is_some()
        {
            self.backend
                .fileview_change(ChangeDirection::ToChild(dir))
                .unwrap();
            self.fileview_refresh(None);
        }
    }

    pub fn fileview_leave(&mut self) {
        if self.fileview.leave() {
            self.backend
                .fileview_change(ChangeDirection::ToParent)
                .unwrap();
            self.fileview_refresh(Some(self.fileview.current.clone()));
        }
    }

    // tag view

    pub fn sync_tags(&self) {
        self.backend.sync_tags();
    }

    pub fn tagview(&mut self) -> &mut View {
        &mut self.tagview
    }

    pub fn tagview_refresh(&mut self, select: Option<String>) {
        let current = self.backend.tagview_current().pop().unwrap_or_default();
        let content = match self.backend.tagview_content() {
            TagViewContent::Genres(v)
            | TagViewContent::Artists(v)
            | TagViewContent::Albums(v)
            | TagViewContent::Titles(v) => v,
        };
        self.tagview = View::new(current, content, select);
    }

    pub fn tagview_enter(&mut self) {
        if let Some(dir) = self.tagview.enter()
            && let TagViewContent::Genres(v)
            | TagViewContent::Artists(v)
            | TagViewContent::Albums(v) = self.backend.tagview_content()
            && v.iter().find(|f| *f == dir).is_some()
        {
            self.backend
                .tagview_change(ChangeDirection::ToChild(dir))
                .unwrap();
            self.tagview_refresh(None);
        }
    }

    pub fn tagview_leave(&mut self) {
        if self.tagview.leave() {
            self.backend
                .tagview_change(ChangeDirection::ToParent)
                .unwrap();
            self.tagview_refresh(Some(self.tagview.current.clone()));
        }
    }

    // misc

    pub fn tick(&mut self) {
        self.throbber_state.calc_next();
        self.toast_engine().tick();
        self.last_tick = Instant::now();
    }

    pub fn last_tick(&self) -> Instant {
        self.last_tick
    }

    pub fn throbbing(&self) -> bool {
        self.throbbing
    }

    pub fn set_throbbing(&mut self, val: bool) {
        self.throbbing = val;
    }

    pub fn throbber_state(&self) -> ThrobberState {
        self.throbber_state.clone()
    }

    pub fn gauge(&self) -> Option<u16> {
        self.gauge
    }

    pub fn set_gauge(&mut self, val: Option<u16>) {
        self.gauge = val;
    }

    pub fn set_toast(&mut self, text: String) {
        self.toast_engine.show_toast(
            ToastBuilder::new(text.into())
                .preset(ToastPreset::CompactHighlightStart, "Error:")
                .toast_type(ToastType::Error)
                .position(ToastPosition::TopRight)
                .offset(0, 1),
        );
    }

    pub fn toast_engine(&mut self) -> &mut ToastEngine<()> {
        &mut self.toast_engine
    }
}

impl View {
    fn new(current: String, content: Vec<String>, select: Option<String>) -> Self {
        let list_state = if content.is_empty() {
            ListState::default()
        } else if let Some(sel) = &select
            && let Some(idx) = content.iter().position(|s| s == sel)
        {
            ListState::default().with_selected(Some(idx))
        } else {
            ListState::default().with_selected(Some(0))
        };
        Self {
            current,
            content,
            list_state,
        }
    }

    pub fn current(&self) -> &str {
        &self.current
    }

    pub fn content(&self) -> Vec<&str> {
        self.content.iter().map(AsRef::as_ref).collect()
    }

    pub fn list_state(&self) -> &ListState {
        &self.list_state
    }

    pub fn select_next(&mut self) {
        if let Some(idx) = self.list_state.selected()
            && idx + 1 < self.content.len()
        {
            self.list_state.select_next();
        }
    }

    pub fn select_prev(&mut self) {
        if let Some(idx) = self.list_state.selected()
            && idx > 0
        {
            self.list_state.select_previous();
        }
    }

    fn enter(&mut self) -> Option<&String> {
        if let Some(idx) = self.list_state.selected()
            && let Some(dir) = self.content.get(idx)
        {
            Some(dir)
        } else {
            None
        }
    }

    fn leave(&mut self) -> bool {
        !self.current.is_empty()
    }
}
