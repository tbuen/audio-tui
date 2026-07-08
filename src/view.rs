use ratatui::Frame;
use ratatui::style::Stylize;
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Paragraph};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let title = Line::from(" Counter App Tutorial ".bold());
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
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let counter_text = Text::from(vec![Line::from(vec![
        "Value: ".into(),
        model.counter().to_string().yellow(),
    ])]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);

    frame.render_widget(paragraph, frame.area());
}
