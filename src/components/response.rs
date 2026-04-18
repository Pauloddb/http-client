use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{
        Block, BorderType, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState,
    },
};

pub fn render_response(app: &mut App, frame: &mut Frame, area: Rect, is_focused: bool) {
    let border_style = if is_focused {
        Style::default()
            .fg(app.config.accent_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.config.default_color)
    };

    let block = Block::default()
        .title("Response")
        .title_bottom(Line::from("<Ctrl + F1> for help").centered())
        .borders(Borders::ALL)
        .border_type(app.config.border_type)
        .border_style(border_style);

    let inner = block.inner(area);

    // Tenta formatar JSON
    let content = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&app.response) {
        serde_json::to_string_pretty(&json).unwrap_or_else(|_| app.response.clone())
    } else {
        app.response.clone()
    };

    // Scrollbar
    let clone = content.clone();
    let lines: Vec<&str> = clone.lines().collect();
    let visible_lines = inner.height as usize;

    app.response_scrollbar_state =
        ScrollbarState::new(lines.len()).position(app.response_scroll.0 as usize);

    let paragraph = Paragraph::new(content)
        .block(block)
        .scroll(app.response_scroll); // (y_offset, x_offset)

    frame.render_widget(paragraph, area);

    // Renderiza scrollbar vertical se necessário
    if lines.len() > visible_lines {
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        frame.render_stateful_widget(
            scrollbar,
            inner, // área interna do block
            &mut app.response_scrollbar_state,
        );
    }
}

pub fn handle_response_scroll(app: &mut App, key: crossterm::event::KeyEvent) {
    use crossterm::event::KeyCode;

    match key.code {
        // Vertical
        KeyCode::Up => {
            app.response_scroll.0 = app.response_scroll.0.saturating_sub(1);
        }
        KeyCode::Down => {
            app.response_scroll.0 = app.response_scroll.0.saturating_add(1);
        }
        // Horizontal
        KeyCode::Left => {
            app.response_scroll.1 = app.response_scroll.1.saturating_sub(1);
        }
        KeyCode::Right => {
            app.response_scroll.1 = app.response_scroll.1.saturating_add(1);
        }
        _ => {}
    }
}
