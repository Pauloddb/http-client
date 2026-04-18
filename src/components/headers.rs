use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, Clear, Paragraph},
};

pub fn render_headers(
    app: &mut App,
    frame: &mut Frame<'_>,
    popup_area: Rect,
    background_color: Color,
) {
    if app.is_headers_focused {
        let headers_block = Block::default()
            .borders(Borders::ALL)
            .border_type(app.config.border_type)
            .title(Line::from("Headers").centered())
            .style(
                Style::default()
                    .fg(app.config.accent_color)
                    .bg(background_color),
            );

        frame.render_widget(Clear, popup_area);
        frame.render_widget(headers_block, popup_area);
    }
}

pub fn handle_headers_input(app: &mut App, key: crossterm::event::KeyEvent) {
    use crossterm::event::{KeyCode, KeyModifiers};

    if app.is_headers_focused && key.modifiers.contains(KeyModifiers::CONTROL) {
        match key.code {
            KeyCode::Char('a') => {}
            KeyCode::Char('d') => {}
            _ => {}
        }
    }
}
