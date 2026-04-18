use ratatui::{style::Color, widgets::BorderType};

pub struct Config {
    pub accent_color: Color,
    pub default_color: Color,
    pub background_color: Color,
    pub border_type: BorderType,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            accent_color: Color::Yellow,
            default_color: Color::Gray,
            background_color: Color::Rgb(20, 20, 20),
            border_type: BorderType::Rounded,
        }
    }
}

pub fn load_config() -> Config {
    Config::default()
}
