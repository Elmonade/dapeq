use iced::{Application, Pixels, Settings, Size};

mod commands;
mod scaffolding;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: Size::new(128., 128.),
        position: Default::default(),
        resizable: false,
        exit_on_close_request: true,
        ..Default::default()
    };

    let settings = Settings {
        id: None,
        window: window_settings,
        flags: (),
        fonts: vec![],
        default_font: Default::default(),
        default_text_size: Pixels(12.),
        antialiasing: false,
        // ..Settings::default()
    };

    scaffolding::DapEq::run(settings)
}
