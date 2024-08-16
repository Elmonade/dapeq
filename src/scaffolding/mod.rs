use std::sync::mpsc;
use std::thread;
use iced::{Application, Command, Font};
use iced::{widget::{column, row, button, Text}};
use rodio::{OutputStream, Sink};
use crate::commands;
use iced_aw;

#[derive(Debug, Clone)]
pub enum Control {
    PlayPause,
    Forward,
    Backward,
    GoBack,
}

pub struct DapEq {
    sender: mpsc::Sender<Control>,
}

// ICONS
const ICON: Font = Font::with_name("icons");

enum Icon {
    User,
    Heart,
    Calc,
    CogAlt,
}

impl From<Icon> for char {
    fn from(icon: Icon) -> Self {
        match icon {
            Icon::User => '\u{E800}',
            Icon::Heart => '\u{E801}',
            Icon::Calc => '\u{F1EC}',
            Icon::CogAlt => '\u{E802}',
        }
    }
}

impl Application for DapEq {
    type Executor = iced::executor::Default;
    type Message = Control;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (DapEq, Command<Self::Message>) {
        let (sender, receiver) = mpsc::channel();
        let _ = thread::Builder::new()
            .name("music_player".to_string())
            .spawn(move || {
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();
                loop {
                    if let Ok(command) = receiver.try_recv() {
                        commands::audio_command(command, &sink);
                    }
                    thread::sleep(std::time::Duration::from_millis(100));
                }
            });
        (DapEq {sender}, Command::none())
    }

    fn title(&self) -> String {
        String::from("DapEq")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Control::PlayPause => self.sender
                .send(Control::PlayPause)
                .expect("Couldn't send audio command."),
            Control::Forward => self.sender
                .send(Control::Forward)
                .expect("Can not skip forward."),
            Control::Backward => self.sender
                .send(Control::Backward)
                .expect("Not implemented, yet."),
            Control::GoBack => self.sender
                .send(Control::GoBack)
                .expect("Not implemented, yet.")
        }
        Command::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        column![
            button("<-").on_press(Self::Message::GoBack),
            row![
                button("<<").on_press(Self::Message::Backward),
                row![
                    button(">/||").on_press(Self::Message::PlayPause),
                ],
                button(">>").on_press(Self::Message::Forward),
            ]
        ].into()
    }
}

