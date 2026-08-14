use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Stylize};
use ratatui::symbols::border;
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, List, Paragraph};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let layout = Layout::horizontal([
        Constraint::Min(40),
        Constraint::Percentage(100),
        Constraint::Min(40),
    ]);

    let [left, _, right] = layout.areas(frame.area());

    // left

    let mut current = model.current().to_owned();
    if !current.is_empty() {
        current.insert(0, ' ');
        current.push(' ');
    }

    let items = model.files();

    let mut state = *model.list_state();

    let sel = match state.selected() {
        Some(idx) => idx + 1,
        None => 0,
    };

    let block = Block::bordered()
        .title(current.bold())
        .title_bottom(Line::from(format!(" {} of {} ", sel, items.len())).right_aligned())
        .border_set(border::THICK);

    let list = List::new(items)
        .style(Color::Black)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(block);

    frame.render_stateful_widget(list, left, &mut state);

    // right

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
        //model.counter().to_string().yellow(),
    ])]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);

    frame.render_widget(paragraph, right);

    //frame.render_widget(paragraph, frame.area());
}
