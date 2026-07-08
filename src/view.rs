use ratatui::Frame;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let title = Line::from(format!(" {} {} ", env!("CARGO_PKG_NAME"), env!("VERSION")).bold());
    let con_state = if model.connected() {
        //Line::from(" \u{f0c52} ".green())
        Line::from(" \u{f0132} ".green())
    } else {
        //Line::from(" \u{f0131} ".red())
        Line::from(" \u{f012e} ".red())
    };
    let instructions = Line::from(vec![
        " Decrement ".into(),
        "<Left>".blue().bold(),
        " Increment ".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
        .title(title.centered())
        .title(con_state.right_aligned())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        model.counter().to_string().yellow(),
    ])]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);

    frame.render_widget(paragraph, frame.area());
}
