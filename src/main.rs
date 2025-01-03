mod commands;
mod scaffolding;

use iced::{Application, Font, Pixels, Settings, Size};
use scaffolding::DapEq;

fn main() -> iced::Result {
    const TERMINUS: Font = Font::with_name("TerminusTTF-4.49.3");
    let window_settings = iced::window::Settings {
        // size: Size::new(128., 128.),
        position: Default::default(),
        // resizable: false,
        exit_on_close_request: true,
        ..Default::default()
    };

    let settings = Settings {
        id: None,
        window: window_settings,
        flags: (),
        fonts: vec![
            include_bytes!("fonts/play_button.ttf")
            .as_slice()
            .into(),
            include_bytes!("fonts/terminus-ttf-4.49.3/TerminusTTF-4.49.3.ttf")
            .as_slice()
            .into(),
        ],
        default_font: TERMINUS,
        default_text_size: Pixels(12.),
        antialiasing: false,
        // ..Settings::default()
    };

    DapEq::run(settings)
}
