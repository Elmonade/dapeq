use iced::{widget::{button, column, row}, Length, Sandbox, Settings, Pixels, window, Size};
mod commands;

fn main() -> iced::Result {
    let set = Settings {
        id: None,
        window: window::Settings {
            size: Size::new(128., 128.),
            position: Default::default(),
            min_size: None,
            max_size: None,
            visible: false,
            resizable: false,
            decorations: false,
            transparent: false,
            level: Default::default(),
            icon: None,
            platform_specific: Default::default(),
            exit_on_close_request: true,
        },
        flags: (),
        fonts: vec![],
        default_font: Default::default(),
        default_text_size: Pixels(6.),
        antialiasing: true,
    };
    DapEq::run(set)
}
#[derive(Debug, Clone)]
enum DapEqCommands {
    Play,
    Pause,
    Forward,
    Backward,
    GoBack,
}

struct DapEq;

impl Sandbox for DapEq {
    type Message = DapEqCommands;

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("DapEq")
    }

    fn update(&mut self, _message: Self::Message) {
        match _message {
            DapEqCommands::Play => {
                commands::play();
            }
            DapEqCommands::Pause => {}
            DapEqCommands::Forward => {}
            DapEqCommands::Backward => {}
            DapEqCommands::GoBack => {}
        }
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            row![
                button(">").on_press(Self::Message::Play),
                button("||").on_press(Self::Message::Pause),
            ],
            row![
                button("<<").on_press(Self::Message::Backward),
                button(">>").on_press(Self::Message::Forward),
                button("<-").on_press(Self::Message::GoBack),
            ]
        ]
        .into()
    }
}
