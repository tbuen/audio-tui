mod event;
mod model;
mod update;
mod view;

use model::Model;

fn main() {
    let mut model = Model::default();

    ratatui::run(|terminal| {
        while !model.wants_exit() {
            terminal.draw(|frame| view::view(&model, frame)).unwrap();
            let msg = event::handle_event(&model);
            update::update(&mut model, msg);
        }
    });
}
