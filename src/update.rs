use crate::model::Model;

pub enum Message {
    ConnectionState(bool),
    FileSyncStarted,
    FileSyncCompleted,
    FileSyncAborted,
    TagSyncStarted,
    TagSyncCompleted,
    TagSyncAborted,
    TagSyncStep(usize, usize),
    SyncFiles,
    SyncTags,
    NextFile,
    PrevFile,
    EnterDir,
    LeaveDir,
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
            model.refresh_file_list();
        }
        Message::FileSyncAborted => model.set_throbbing(false),
        Message::TagSyncStarted => model.set_gauge(Some(0)),
        Message::TagSyncCompleted => {
            model.set_gauge(None);
            //model.refresh_tag_list();
        }
        Message::TagSyncAborted => model.set_gauge(None),
        #[allow(clippy::cast_possible_truncation)]
        Message::TagSyncStep(n, t) => model.set_gauge(Some((n * 100 / t) as u16)),
        Message::SyncFiles => model.sync_files(),
        Message::SyncTags => model.sync_tags(),
        Message::NextFile => model.select_next(),
        Message::PrevFile => model.select_prev(),
        Message::EnterDir => model.enter_dir(),
        Message::LeaveDir => model.leave_dir(),
        Message::Error(s) => model.set_toast(s),
        Message::Quit => model.set_exit(),
        Message::Resize => {}
        Message::Tick => model.tick(),
    }
}
