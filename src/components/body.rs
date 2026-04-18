use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};
use ratatui_textarea::TextArea;

pub fn render_body(app: &mut App, frame: &mut Frame, area: Rect, is_focused: bool) {
    let border_style = if is_focused {
        Style::default()
            .fg(app.config.accent_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(app.config.default_color)
    };

    let block = Block::default()
        .title("Body")
        .borders(Borders::ALL)
        .border_type(app.config.border_type)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // ✅ &self.body_editor funciona porque &TextArea implementa Widget [^14^]
    frame.render_widget(&app.body_editor, inner);
}

pub fn format_body_json(app: &mut App) {
    let content = app.body_editor.lines().join("\n");

    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
        let formatted = serde_json::to_string_pretty(&json).unwrap_or_default();

        // ✅ TextArea::from() aceita iterator de String [^14^]
        let lines: Vec<String> = formatted.lines().map(|s| s.to_string()).collect();

        // Recria preservando configurações
        let mut new_editor = TextArea::from(lines);
        new_editor.set_tab_length(2);
        new_editor.set_hard_tab_indent(true);
        new_editor.set_placeholder_text("{\n  \"key\": \"value\"\n}");
        new_editor.set_placeholder_style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        );
        new_editor.set_selection_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
        new_editor.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        app.body_editor = new_editor;
    }
}
