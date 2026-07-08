mod event;
mod model;
mod update;
mod view;

use model::Model;

fn main() {
    let mut model = Model::new();

    ratatui::run(|terminal| {
        while !model.exit() {
            terminal.draw(|frame| view::view(&model, frame)).unwrap();
            let msg = event::handle_event(&model);
            update::update(&mut model, msg);
        }
    });
}
