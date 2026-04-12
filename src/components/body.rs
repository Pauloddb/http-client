use crate::app::App;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders},
};

pub fn render_body(app: &mut App, frame: &mut Frame, area: Rect, is_focused: bool) {
    let border_style = if is_focused {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let block = Block::default()
        .title("Body")
        .borders(Borders::ALL)
        .border_style(border_style);

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // ✅ &self.body_editor funciona porque &TextArea implementa Widget [^14^]
    frame.render_widget(&app.body_editor, inner);
}
