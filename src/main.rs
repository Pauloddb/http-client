mod app;
mod components;
mod config;
mod models;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();

    let config = config::load_config();
    let mut my_app = app::App::new(config);

    let result = my_app.run(&mut terminal).await;

    ratatui::restore(); // Sempre restaura, mesmo se houver erro
    result
}
