mod app;
mod components;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // NOVA API simplificada!
    let mut terminal = ratatui::init();
    let mut app = app::App::new();

    let result = app::run_app(&mut terminal, &mut app).await;

    ratatui::restore(); // Sempre restaura, mesmo se houver erro
    result
}
