use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use iced::{Application, Pixels, Settings, Size, window, Command};
use iced::{widget::{column, button, row}};
use rodio::{OutputStream, Sink};

mod commands;

fn main() -> iced::Result {
    let window_settings = iced::window::Settings {
        size: Size::new(128., 128.),
        exit_on_close_request: true,
        ..Default::default()
    };

    let settings = Settings {
        window: window_settings,
        ..Settings::default()
    };

    DapEq::run(settings)
}

#[derive(Debug, Clone)]
enum Control {
    Play,
    Pause,
    Forward,
    Backward,
    GoBack,
}

struct DapEq {
    file_path: Option<PathBuf>,
    audio_command_sender: mpsc::Sender<Control>,
}

impl Application for DapEq {
    type Executor = iced::executor::Default;
    type Message = Control;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (DapEq, Command<Self::Message>) {
        let (sender, receiver) = mpsc::channel();

        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let sink = Sink::try_new(&stream_handle).unwrap();

            loop {
                if let Ok(command) = receiver.try_recv() {
                    audio_command(command, &sink);
                }
                thread::sleep(std::time::Duration::from_millis(100));
            }
        });

        (
            DapEq {
                file_path: None,
                audio_command_sender: sender,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DapEq")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Control::Play => self.audio_command_sender
                .send(Control::Play)
                .expect("Couldn't send audio command."),
            Control::Pause => self.audio_command_sender
                .send(Control::Pause)
                .expect("Couldn't send audio command."),
            Control::Forward => {}
            Control::Backward => {}
            Control::GoBack => {}
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
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

fn audio_command(_message: Control, sink: &Sink) {
    match _message {
        Control::Play => commands::play(sink),
        Control::Pause => commands::pause(sink),
        Control::Forward => {}
        Control::Backward => {}
        Control::GoBack => {}
    }
}
