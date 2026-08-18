use ratatui::Frame;
use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::symbols::{border, line};
use ratatui::text::{Line, Span, Text};
use ratatui::widgets::{Block, LineGauge, List, Paragraph};
use throbber_widgets_tui::Throbber;

use crate::model::Model;

pub fn view(model: &mut Model, frame: &mut Frame) {
    let layout = Layout::vertical([Constraint::Min(1), Constraint::Percentage(100)]);
    let [top_area, main_area] = layout.areas(frame.area());

    // top

    let title = Span::from(format!(
        "{} {}",
        env!("CARGO_PKG_NAME"),
        env!("VERSION").bold(),
    ));

    let con_state = if model.connected() {
        "\u{f0132}".green()
    } else {
        "\u{f012e}".red()
    };

    let layout = Layout::horizontal([
        Constraint::Min(1),
        Constraint::Percentage(100),
        #[allow(clippy::cast_possible_truncation)]
        Constraint::Min(title.width() as u16),
        Constraint::Min(1),
    ])
    .horizontal_margin(1)
    .spacing(1);

    let [throb_area, gauge_area, title_area, con_area] = layout.areas(top_area);

    if model.throbbing() {
        let throb = Throbber::default()
            .throbber_set(throbber_widgets_tui::BRAILLE_EIGHT_DOUBLE)
            .style(Style::default().bold());
        frame.render_stateful_widget(throb, throb_area, &mut model.throbber_state());
    }

    if let Some(p) = model.gauge() {
        let gauge = LineGauge::default()
            .filled_symbol(line::THICK_HORIZONTAL)
            .ratio(f64::from(p) / 100.0);
        frame.render_widget(gauge, gauge_area);
    }

    frame.render_widget(title, title_area);
    frame.render_widget(con_state, con_area);

    // main

    let layout = Layout::horizontal([
        Constraint::Min(40),
        Constraint::Percentage(100),
        Constraint::Min(40),
    ]);

    let [left, _, right] = layout.areas(main_area);

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
        .title_bottom(Line::from(format!(" {}/{} ", sel, items.len())).right_aligned())
        .border_set(border::THICK);

    let list = List::new(items)
        .style(Color::Black)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(block);

    frame.render_stateful_widget(list, left, &mut state);

    // right

    let instructions = Line::from(vec![
        " Decrement ".into(),
        "<Left>".blue().bold(),
        " Increment ".into(),
        "<Right>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    let block = Block::bordered()
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
