use crate::models::method::HttpMethod;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub struct MethodSelector {
    method: HttpMethod,
    is_focused: bool,
}

impl MethodSelector {
    pub fn new() -> Self {
        Self {
            method: HttpMethod::default(),
            is_focused: false,
        }
    }

    pub fn method(&self) -> HttpMethod {
        self.method
    }

    pub fn next(&mut self) {
        self.method = self.method.next();
    }

    pub fn previous(&mut self) {
        self.method = self.method.previous();
    }

    pub fn set_focus(&mut self, focused: bool) {
        self.is_focused = focused;
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let border_style = if self.is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .title(" Method ")
            .style(border_style);

        let text = if self.is_focused {
            Line::from(vec![
                Span::styled("< ", Style::default().fg(Color::Gray)),
                Span::styled(
                    self.method.to_string(),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" >", Style::default().fg(Color::Gray)),
            ])
        } else {
            Line::from(Span::styled(
                self.method.to_string(),
                Style::default().fg(Color::White),
            ))
        };

        let paragraph = Paragraph::new(text).block(block).centered();
        frame.render_widget(paragraph, area);
    }
}
