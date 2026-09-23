use crate::model::{Model, Pane};

pub enum Message {
    ConnectionState(bool),
    FileSyncStarted,
    FileSyncCompleted,
    FileSyncAborted,
    TagSyncStarted,
    TagSyncCompleted,
    TagSyncAborted,
    TagSyncStep(usize, usize),
    ShowFilesPane,
    ShowTagsPane,
    SyncFiles,
    SyncTags,
    FileNext,
    FilePrev,
    FileEnter,
    FileLeave,
    TagNext,
    TagPrev,
    TagEnter,
    TagLeave,
    Error(String),
    Quit,
    Resize,
    Tick,
}

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::ConnectionState(value) => model.set_connected(value),
        Message::FileSyncStarted => model.set_throbbing(true),
        Message::FileSyncCompleted => {
            model.set_throbbing(false);
            model.fileview_refresh(None);
        }
        Message::FileSyncAborted => model.set_throbbing(false),
        Message::TagSyncStarted => model.set_gauge(Some(0)),
        Message::TagSyncCompleted => {
            model.set_gauge(None);
            model.tagview_refresh(None);
        }
        Message::TagSyncAborted => model.set_gauge(None),
        #[allow(clippy::cast_possible_truncation)]
        Message::TagSyncStep(n, t) => model.set_gauge(Some((n * 100 / t) as u16)),
        Message::ShowFilesPane => model.set_active_pane(Pane::Files),
        Message::ShowTagsPane => model.set_active_pane(Pane::Tags),
        Message::SyncFiles => model.sync_files(),
        Message::SyncTags => model.sync_tags(),
        Message::FileNext => model.fileview().select_next(),
        Message::FilePrev => model.fileview().select_prev(),
        Message::FileEnter => model.fileview_enter(),
        Message::FileLeave => model.fileview_leave(),
        Message::TagNext => model.tagview().select_next(),
        Message::TagPrev => model.tagview().select_prev(),
        Message::TagEnter => model.tagview_enter(),
        Message::TagLeave => model.tagview_leave(),
        Message::Error(s) => model.set_toast(s),
        Message::Quit => model.set_exit(),
        Message::Resize => {}
        Message::Tick => model.tick(),
    }
}
