use crate::model::Model;

pub enum Message {
    ConnectionState(bool),
    FilesSynced,
    SyncFiles,
    NextFile,
    PrevFile,
    EnterDir,
    LeaveDir,
    Quit,
    Resize,
}

pub fn update(model: &mut Model, msg: Message) {
    match msg {
        Message::ConnectionState(value) => model.set_connected(value),
        Message::FilesSynced => model.refresh_file_list(),
        Message::SyncFiles => model.sync_files(),
        Message::NextFile => model.select_next(),
        Message::PrevFile => model.select_prev(),
        Message::EnterDir => model.enter_dir(),
        Message::LeaveDir => model.leave_dir(),
        Message::Quit => model.set_exit(),
        Message::Resize => {}
    }
}
