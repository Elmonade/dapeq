use iced::{widget::{button, column, row}, Length, Sandbox, Settings, Pixels, window, Size};

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
    MyApp::run(set)
}

struct MyApp;

impl Sandbox for MyApp {
    type Message = ();

    fn new() -> Self {
        Self
    }

    fn title(&self) -> String {
        String::from("My App")
    }

    fn update(&mut self, _message: Self::Message) {}

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            button("Shrink").width(Length::Shrink),
            button("Fill").width(Length::Fill),
            row![
                button("FillPortion 2").width(Length::FillPortion(2)),
                button("FillPortion 1").width(Length::FillPortion(1)),
            ]
            .spacing(10),
            button("Fixed").width(Length::Fixed(100.)),
            button("Fill (height)").height(Length::Fill),
        ]
            .spacing(10)
            .into()
    }
}
