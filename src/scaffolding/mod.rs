mod icons;

use std::sync::mpsc;
use std::thread;
use iced::{Application, Command};
use iced::{widget::{column, row}};
use rodio::{OutputStream, Sink};
use crate::commands;

#[derive(Debug, Clone)]
pub enum Control {
    PlayPause,
    Forward,
    Backward,
    GoBack,
}

pub enum State {
    Playing,
    Paused,
}

pub struct DapEq {
    sender: mpsc::Sender<Control>,
    state: State,
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
                sender,
                state:State::Paused,
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("DapEq")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        match _message {
            Control::PlayPause => {
                self.sender
                    .send(Control::PlayPause)
                    .expect("Couldn't send audio command.");
                self.state = match self.state {
                    State::Playing => State::Paused,
                    State::Paused => State::Playing,
                }
            },
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

    fn view(&self) -> iced::Element<'_, Control> {
        let flip_button = match self.state {
            State::Paused =>
                icons::play_button(),
            State::Playing =>
                icons::pause_button(),

        };

        column![
            icons::action(icons::go_back_button(), Control::GoBack),
            row![
                icons::action(icons::backward_button(), Control::Backward),
                icons::action(flip_button, Control::PlayPause),
                icons::action(icons::forward_button(), Control::Forward),
            ]
        ].into()
    }
}
