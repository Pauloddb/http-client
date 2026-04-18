use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, BorderType, Borders, Paragraph},
};

pub fn render_help(app: &App, frame: &mut Frame, area: Rect, background_color: Color) {
    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .border_type(app.config.border_type)
        .border_style(Style::default().fg(app.config.accent_color))
        .style(Style::default().bg(background_color)); // ← bg no block

    // Paragraph com bg no style principal
    let text_style = Style::default().bg(background_color);

    let content = Text::from(vec![
        Line::from("Use <Ctrl + ?> to toggle help."),
        Line::from("Previous field: <Ctrl + P>"),
        Line::from("Next field: <Ctrl + N>"),
        Line::from("Toggle headers popup: <Ctrl + H>"),
        Line::from("Quit: <Ctrl + Q>"),
        Line::from("Format JSON: <Ctrl + F>"),
        Line::from("Send request: <Ctrl + R>"),
    ]);

    // Aplica bg em tudo: block e paragraph
    frame.render_widget(
        Paragraph::new(content).block(block).style(text_style), // ← bg no paragraph
        area,
    );
}
