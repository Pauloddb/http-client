use std::collections::HashMap;

use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    style::Stylize,
};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
};
use reqwest::{Client, Response, header::HeaderMap};

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
    body: String,
    body_cursor: usize,
    headers: HashMap<String, String>,
    response: String,
    client: Client,
}

impl App {
    pub fn new() -> Self {
        Self {
            focus: FocusManager::new(),
            url: String::new(),
            url_cursor: 0,
            method_selector: MethodSelector::new(),
            body: String::new(),
            body_cursor: 0,
            headers: HashMap::new(),
            response: String::from("Aguardando requisição..."),
            client: Client::new(),
        }
    }

    pub fn render_input(
        &self,
        area: Rect,
        title: &str,
        content: &str,
        frame: &mut Frame,
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
                "Digite uma url...",
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

    pub fn render_text_area(
        &self,
        frame: &mut Frame,
        area: Rect,
        title: &str,
        content: &str,
        is_focused: bool,
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

        let paragraph = Paragraph::new(content).block(block);

        frame.render_widget(paragraph, area);
    }
}

pub async fn run_app(
    terminal: &mut ratatui::DefaultTerminal,
    app: &mut App,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            let chunks = Layout::vertical([Constraint::Length(3), Constraint::Fill(1)]).split(area);

            let up_chunks =
                Layout::horizontal([Constraint::Fill(1), Constraint::Length(15)]).split(chunks[0]);

            let down_chunks =
                Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).split(chunks[1]);

            // Rendering
            app.render_input(
                up_chunks[0],
                " URL ",
                app.url.as_str(),
                frame,
                app.focus.is_focused(Field::Url),
                app.url_cursor,
            );

            app.render_text_area(
                frame,
                down_chunks[0],
                " Body ",
                app.body.as_str(),
                app.focus.is_focused(Field::Body),
            );
            app.render_text_area(
                frame,
                down_chunks[1],
                " Response ",
                app.response.as_str(),
                app.focus.is_focused(Field::Response),
            );

            app.method_selector
                .set_focus(app.focus.is_focused(Field::Method));
            app.method_selector.render(frame, up_chunks[1]);
        })?;

        // Event handling
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            if key.modifiers == KeyModifiers::CONTROL {
                match key.code {
                    KeyCode::Char('n') => {
                        app.focus.next();
                        app.method_selector
                            .set_focus(app.focus.is_focused(Field::Method));
                    }
                    KeyCode::Char('p') => {
                        app.focus.previous();
                        app.method_selector
                            .set_focus(app.focus.is_focused(Field::Method));
                    }
                    KeyCode::Char('r') => {
                        app.response = String::from("Processando requisição...");
                        handle_request(app).await;
                    }
                    KeyCode::Char('q') => break Ok(()),
                    _ => {}
                }
            }

            match app.focus.current() {
                Field::Url => handle_url_input(app, key),
                Field::Body => handle_body_input(app, key),
                Field::Headers => {}
                Field::Response => {}
                Field::Method => match key.code {
                    KeyCode::Left => app.method_selector.previous(),
                    KeyCode::Right => app.method_selector.next(),
                    _ => {}
                },
            }
        }
    }
}

fn handle_url_input(app: &mut App, key: event::KeyEvent) {
    if key.modifiers.contains(KeyModifiers::CONTROL) {
        return;
    }

    match key.code {
        KeyCode::Char(c) => {
            // Insere na posição do cursor, não no final
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
        KeyCode::Home => {
            app.url_cursor = 0;
        }
        KeyCode::End => {
            app.url_cursor = app.url.len();
        }
        _ => {}
    }
}

fn handle_body_input(app: &mut App, key: event::KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            // Insere na posição do cursor, não no final
            app.body.insert(app.body_cursor, c);
            app.body_cursor += 1;
        }
        KeyCode::Backspace => {
            if app.body_cursor > 0 {
                app.body_cursor -= 1;
                app.body.remove(app.body_cursor);
            }
        }
        KeyCode::Delete => {
            if app.body_cursor < app.body.len() {
                app.body.remove(app.body_cursor);
            }
        }
        KeyCode::Left => {
            if app.body_cursor > 0 {
                app.body_cursor -= 1;
            }
        }
        KeyCode::Right => {
            if app.body_cursor < app.body.len() {
                app.body_cursor += 1;
            }
        }
        KeyCode::Home => {
            app.body_cursor = 0;
        }
        KeyCode::End => {
            app.body_cursor = app.body.len();
        }
        KeyCode::Enter => {
            app.body.insert(app.body_cursor, '\n');
            app.body_cursor += 1;
        }
        KeyCode::Tab => {
            app.body.insert_str(app.body_cursor, "    ");
            app.body_cursor += 1;
        }
        _ => {}
    }
}

async fn handle_request(app: &mut App) {
    // Cria builder base
    let mut req = match app.method_selector.method().as_str() {
        "GET" => app.client.get(&app.url),
        "POST" => app.client.post(&app.url),
        "DELETE" => app.client.delete(&app.url),
        "PUT" => app.client.put(&app.url),
        "PATCH" => app.client.patch(&app.url),
        _ => {
            app.response = format!("Método não suportado: {}", app.method_selector.method());
            return;
        }
    };

    // Adiciona headers se houver
    for (key, value) in &app.headers {
        req = req.header(key, value);
    }

    // Adiciona body para métodos que suportam
    match app.method_selector.method().as_str() {
        "POST" | "PUT" | "PATCH" => {
            req = req.body(app.body.clone());
        }
        _ => {}
    }

    // Envia e processa resposta
    match req.send().await {
        Ok(res) => {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            app.response = format!("Status: {}\n\n{}", status, text);
        }
        Err(e) => {
            app.response = format!("Erro: {}", e);
        }
    }
}
