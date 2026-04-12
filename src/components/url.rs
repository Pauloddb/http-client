use ratatui::{
    Frame,
    layout::{Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn render_url(
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
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .borders(Borders::ALL)
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
