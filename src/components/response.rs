use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render_response(app: &App, frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .title("Response")
        .title_bottom(Line::from("<Ctrl + F1> for help").centered())
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    // Tenta formatar JSON
    let content = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&app.response) {
        serde_json::to_string_pretty(&json).unwrap_or_else(|_| app.response.clone())
    } else {
        app.response.clone()
    };

    frame.render_widget(
        Paragraph::new(content)
            .block(block)
            .wrap(Wrap { trim: true }),
        area,
    );
}
