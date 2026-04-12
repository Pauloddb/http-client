use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};
use ratatui_textarea::TextArea; // ✅ Confirmado na docs.rs [^34^]
use reqwest::Client;

use crate::{
    components::{
        body::render_body, help::render_help, method_selector::MethodSelector,
        response::render_response, url::render_url,
    },
    models::{
        focus::{Field, FocusManager},
        method::HttpMethod,
    },
};

pub struct App {
    pub focus: FocusManager,
    pub url: String,
    pub url_cursor: usize,
    pub method_selector: MethodSelector,
    pub body_editor: TextArea<'static>,
    pub headers: HashMap<String, String>,
    pub response: String,
    pub client: Client,
    pub is_headers_focused: bool,
    pub is_help_focused: bool,
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
            response: String::from("Waiting for response..."),
            client: Client::new(),
            is_headers_focused: false,
            is_help_focused: false,
        }
    }

    // ✅ &mut self necessário porque TextArea mantém estado interno do cursor

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

                let background_color = Color::Rgb(20, 20, 20);

                let background = Block::default().style(Style::default().bg(background_color));
                frame.render_widget(background, area);

                let chunks =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

                let up_chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(15)])
                    .split(chunks[0]);

                let down_chunks =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(chunks[1]);

                let popup_area =
                    area.centered(Constraint::Percentage(60), Constraint::Percentage(60));

                let headers_block = Block::default()
                    .borders(Borders::ALL)
                    .title(Line::from("Headers").centered())
                    .style(Style::default().fg(Color::Yellow).bg(background_color));

                // URL
                render_url(
                    frame,
                    up_chunks[0],
                    "URL",
                    &self.url,
                    "https://api.example.com/users",
                    self.focus.is_focused(Field::Url)
                        && !self.is_headers_focused
                        && !self.is_help_focused,
                    self.url_cursor,
                );

                // Body - ✅ &mut self necessário
                render_body(
                    self,
                    frame,
                    down_chunks[0],
                    self.focus.is_focused(Field::Body)
                        && !self.is_headers_focused
                        && !self.is_help_focused,
                );

                // Response
                render_response(self, frame, down_chunks[1]);

                // Method
                self.method_selector.set_focus(
                    self.focus.is_focused(Field::Method)
                        && !self.is_headers_focused
                        && !self.is_help_focused,
                );
                self.method_selector.render(frame, up_chunks[1]);

                // Headers
                if self.is_headers_focused {
                    frame.render_widget(Clear, popup_area);
                    frame.render_widget(headers_block, popup_area);
                }

                // Help
                if self.is_help_focused {
                    frame.render_widget(Clear, popup_area);
                    render_help(frame, popup_area, background_color);
                }
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
                        KeyCode::Char('h') => {
                            self.is_help_focused = false;
                            self.is_headers_focused = !self.is_headers_focused;
                            continue;
                        }
                        KeyCode::F(1) => {
                            self.is_headers_focused = false;
                            self.is_help_focused = !self.is_help_focused;
                            continue;
                        }
                        _ => {}
                    }
                }

                // Input por campo
                match self.focus.current() {
                    Field::Url if !self.is_headers_focused && !self.is_help_focused => {
                        self.handle_url_input(key)
                    }
                    Field::Body if !self.is_headers_focused && !self.is_help_focused => {
                        self.handle_body_input(key)
                    }
                    Field::Method if !self.is_headers_focused && !self.is_help_focused => {
                        match key.code {
                            KeyCode::Left if !self.is_headers_focused && !self.is_help_focused => {
                                self.method_selector.previous()
                            }
                            KeyCode::Right if !self.is_headers_focused && !self.is_help_focused => {
                                self.method_selector.next()
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
