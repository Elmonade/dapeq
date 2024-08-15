use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use iced::{Application, Command};
use iced::{widget::{column, row, button}};
use rodio::{OutputStream, Sink};
use crate::commands;

#[derive(Debug, Clone)]
pub enum Control {
    Play,
    Pause,
    Forward,
    Backward,
    GoBack,
}

pub struct DapEq {
    path: Option<PathBuf>,
    sender: mpsc::Sender<Control>,
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

        (
            DapEq {
                path: None,
                sender,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DapEq")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Control::Play => self.sender
                .send(Control::Play)
                .expect("Couldn't send audio command."),
            Control::Pause => self.sender
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

