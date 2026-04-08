mod app;
mod components;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // NOVA API simplificada!
    let mut terminal = ratatui::init();
    let mut my_app = app::App::new();

    let result = my_app.run(&mut terminal).await;

    ratatui::restore(); // Sempre restaura, mesmo se houver erro
    result
}
