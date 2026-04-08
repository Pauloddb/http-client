use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use ratatui_textarea::TextArea; // ✅ Confirmado na docs.rs [^34^]
use reqwest::Client;

use crate::{
    components::method_selector::MethodSelector,
    models::{
        focus::{Field, FocusManager},
        method::HttpMethod,
    },
};

pub struct App {
    focus: FocusManager,
    url: String,
    url_cursor: usize,
    method_selector: MethodSelector,
    body_editor: TextArea<'static>,
    headers: HashMap<String, String>,
    response: String,
    client: Client,
}

impl App {
    pub fn new() -> Self {
        let mut body_editor = TextArea::default(); // ✅ Confirmado [^14^]

        // ✅ set_tab_length confirmado na docs [^14^]
        body_editor.set_tab_length(2);

        // ✅ set_hard_tab_indent confirmado [^14^]
        body_editor.set_hard_tab_indent(true);

        // ✅ set_placeholder_text confirmado [^14^]
        body_editor.set_placeholder_text("{\n  \"key\": \"value\"\n}");

        // ✅ set_placeholder_style confirmado [^14^]
        body_editor.set_placeholder_style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::ITALIC),
        );

        // ✅ set_selection_style confirmado [^14^]
        body_editor.set_selection_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

        // ✅ set_cursor_line_style confirmado [^14^]
        body_editor.set_cursor_line_style(Style::default().add_modifier(Modifier::UNDERLINED));

        Self {
            focus: FocusManager::new(),
            url: String::new(),
            url_cursor: 0,
            method_selector: MethodSelector::new(),
            body_editor,
            headers: HashMap::new(),
            response: String::from(
                "Ctrl+R: enviar | Ctrl+Q: sair | Ctrl+N/P: navegar | Ctrl+F: formatar JSON",
            ),
            client: Client::new(),
        }
    }

    fn render_input(
        &self,
        frame: &mut Frame,
        area: Rect,
        title: &str,
        content: &str,
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
                "https://api.example.com/users",
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

    // ✅ &mut self necessário porque TextArea mantém estado interno do cursor
    fn render_body(&mut self, frame: &mut Frame, area: Rect, is_focused: bool) {
        let border_style = if is_focused {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        let block = Block::default()
            .title(" Body ")
            .borders(Borders::ALL)
            .border_style(border_style);

        let inner = block.inner(area);
        frame.render_widget(block, area);

        // ✅ &self.body_editor funciona porque &TextArea implementa Widget [^14^]
        frame.render_widget(&self.body_editor, inner);
    }

    fn render_response(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Response ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        // Tenta formatar JSON
        let content = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&self.response) {
            serde_json::to_string_pretty(&json).unwrap_or_else(|_| self.response.clone())
        } else {
            self.response.clone()
        };

        frame.render_widget(
            Paragraph::new(content)
                .block(block)
                .wrap(Wrap { trim: true }),
            area,
        );
    }

    // ✅ input() aceita KeyEvent diretamente com feature crossterm [^14^][^34^]
    fn handle_body_input(&mut self, key: crossterm::event::KeyEvent) {
        self.body_editor.input(key); // Conversão automática crossterm → Input
    }

    async fn handle_request(&mut self) {
        let method_str = self.method_selector.method().as_str();

        let mut req = match method_str {
            "GET" => self.client.get(&self.url),
            "POST" => self.client.post(&self.url),
            "DELETE" => self.client.delete(&self.url),
            "PUT" => self.client.put(&self.url),
            "PATCH" => self.client.patch(&self.url),
            _ => {
                self.response = format!("Método não suportado: {}", method_str);
                return;
            }
        };

        // Headers
        for (key, value) in &self.headers {
            req = req.header(key, value);
        }

        // ✅ lines() retorna &[String], join("\n") retorna String [^14^]
        if matches!(method_str, "POST" | "PUT" | "PATCH") {
            let body = self.body_editor.lines().join("\n");
            if !body.is_empty() {
                req = req.body(body);
            }
        }

        match req.send().await {
            Ok(res) => {
                let status = res.status();
                let text = res.text().await.unwrap_or_default();
                self.response = format!("Status: {}\n\n{}", status, text);
            }
            Err(e) => {
                self.response = format!("Erro: {}", e);
            }
        }
    }

    fn handle_url_input(&mut self, key: crossterm::event::KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) {
            return;
        }

        match key.code {
            KeyCode::Char(c) => {
                self.url.insert(self.url_cursor, c);
                self.url_cursor += 1;
            }
            KeyCode::Backspace => {
                if self.url_cursor > 0 {
                    self.url_cursor -= 1;
                    self.url.remove(self.url_cursor);
                }
            }
            KeyCode::Delete => {
                if self.url_cursor < self.url.len() {
                    self.url.remove(self.url_cursor);
                }
            }
            KeyCode::Left => {
                if self.url_cursor > 0 {
                    self.url_cursor -= 1;
                }
            }
            KeyCode::Right => {
                if self.url_cursor < self.url.len() {
                    self.url_cursor += 1;
                }
            }
            KeyCode::Home => self.url_cursor = 0,
            KeyCode::End => self.url_cursor = self.url.len(),
            _ => {}
        }
    }

    fn format_body_json(&mut self) {
        let content = self.body_editor.lines().join("\n");

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

            self.body_editor = new_editor;
        }
    }

    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();

                let chunks =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

                let up_chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(15)])
                    .split(chunks[0]);

                let down_chunks =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(chunks[1]);

                // URL
                self.render_input(
                    frame,
                    up_chunks[0],
                    " URL ",
                    &self.url,
                    self.focus.is_focused(Field::Url),
                    self.url_cursor,
                );

                // Body - ✅ &mut self necessário
                self.render_body(frame, down_chunks[0], self.focus.is_focused(Field::Body));

                // Response
                self.render_response(frame, down_chunks[1]);

                // Method
                self.method_selector
                    .set_focus(self.focus.is_focused(Field::Method));
                self.method_selector.render(frame, up_chunks[1]);
            })?;

            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                // Atalhos globais
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('q') => break Ok(()),
                        KeyCode::Char('r') => {
                            self.handle_request().await;
                            continue;
                        }
                        KeyCode::Char('n') => {
                            self.focus.next();
                            self.method_selector
                                .set_focus(self.focus.is_focused(Field::Method));
                            continue;
                        }
                        KeyCode::Char('p') => {
                            self.focus.previous();
                            self.method_selector
                                .set_focus(self.focus.is_focused(Field::Method));
                            continue;
                        }
                        KeyCode::Char('f') => {
                            if self.focus.is_focused(Field::Body) {
                                self.format_body_json();
                            }
                            continue;
                        }
                        _ => {}
                    }
                }

                // Input por campo
                match self.focus.current() {
                    Field::Url => self.handle_url_input(key),
                    Field::Body => self.handle_body_input(key),
                    Field::Method => match key.code {
                        KeyCode::Left => self.method_selector.previous(),
                        KeyCode::Right => self.method_selector.next(),
                        _ => {}
                    },
                    _ => {}
                }
            }
        }
    }
}
