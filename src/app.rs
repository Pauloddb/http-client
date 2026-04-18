use std::collections::HashMap;

use crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use ratatui_textarea::TextArea;
use reqwest::Client;

use crate::{
    components::{
        body::{format_body_json, render_body},
        headers::{handle_headers_input, render_headers},
        help::render_help,
        method_selector::MethodSelector,
        response::{handle_response_scroll, render_response},
        url::{handle_url_input, render_url},
    },
    config::Config,
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
    pub response_scroll: (u16, u16), // (y, x)
    pub response_scrollbar_state: ScrollbarState,
    pub client: Client,
    pub is_headers_focused: bool,
    pub is_help_focused: bool,
    pub config: Config,
}

impl App {
    pub fn new(config: Config) -> Self {
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
            response_scroll: (0, 0),
            response_scrollbar_state: ScrollbarState::default(),
            client: Client::new(),
            is_headers_focused: false,
            is_help_focused: false,
            config,
        }
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

    pub async fn run(&mut self, terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
        loop {
            terminal.draw(|frame| {
                let area = frame.area();

                let background =
                    Block::default().style(Style::default().bg(self.config.background_color));
                frame.render_widget(background, area);

                let chunks =
                    Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

                let up_chunks = Layout::horizontal([Constraint::Fill(1), Constraint::Length(15)])
                    .split(chunks[0]);

                let down_chunks =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(chunks[1]);

                let popup_area =
                    area.centered(Constraint::Percentage(60), Constraint::Percentage(60));

                // URL
                render_url(
                    self,
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
                render_response(
                    self,
                    frame,
                    down_chunks[1],
                    self.focus.is_focused(Field::Response)
                        && !self.is_headers_focused
                        && !self.is_help_focused,
                );

                // Method
                self.method_selector.set_focus(
                    self.focus.is_focused(Field::Method)
                        && !self.is_headers_focused
                        && !self.is_help_focused,
                );
                self.method_selector
                    .render(&self.config, frame, up_chunks[1]);

                // Headers
                render_headers(self, frame, popup_area, self.config.background_color);

                // Help
                if self.is_help_focused {
                    frame.render_widget(Clear, popup_area);
                    render_help(self, frame, popup_area, self.config.background_color);
                }
            })?;

            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                // Atalhos globais
                if key.modifiers == KeyModifiers::CONTROL {
                    match key.code {
                        KeyCode::Char('q') => {
                            break Ok(());
                        }
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
                                format_body_json(self);
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
                        handle_url_input(self, key)
                    }
                    Field::Body if !self.is_headers_focused && !self.is_help_focused => {
                        self.body_editor.input(key);
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
                    Field::Response if !self.is_headers_focused && !self.is_help_focused => {
                        handle_response_scroll(self, key)
                    }
                    _ => {}
                }
            }
        }
    }
}
