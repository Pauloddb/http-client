use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub fn render_url(
    app: &App,
    frame: &mut Frame,
    area: Rect,
    title: &str,
    content: &str,
    placeholder: &str,
    is_focused: bool,
    cursor_pos: usize,
) {
    let border_style = if is_focused {
        Style::default()
            .fg(app.config.accent_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.config.default_color)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(app.config.border_type)
        .title(title)
        .border_style(border_style);

    let text = if content.is_empty() && is_focused {
        Line::from(vec![Span::styled(
            placeholder,
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        )])
    } else {
        Line::from(content)
    };

    let paragraph = Paragraph::new(text).block(block);
    frame.render_widget(paragraph, area);

    if is_focused {
        let inner_x = area.x + 1 + cursor_pos.min(content.len()) as u16;
        frame.set_cursor_position(Position::new(inner_x, area.y + 1));
    }
}

pub fn handle_url_input(app: &mut App, key: crossterm::event::KeyEvent) {
    use crossterm::event::{KeyCode, KeyModifiers};

    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return;
    }

    match key.code {
        KeyCode::Char(c) => {
            app.url.insert(app.url_cursor, c);
            app.url_cursor += 1;
        }
        KeyCode::Backspace => {
            if app.url_cursor > 0 {
                app.url_cursor -= 1;
                app.url.remove(app.url_cursor);
            }
        }
        KeyCode::Delete => {
            if app.url_cursor < app.url.len() {
                app.url.remove(app.url_cursor);
            }
        }
        KeyCode::Left => {
            if app.url_cursor > 0 {
                app.url_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.url_cursor < app.url.len() {
                app.url_cursor += 1;
            }
        }
        KeyCode::Home => app.url_cursor = 0,
        KeyCode::End => app.url_cursor = app.url.len(),
        _ => {}
    }
}
