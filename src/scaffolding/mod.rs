use std::sync::mpsc;
use std::thread;

use iced::{Application, Command};
use iced::widget::{column, row, text};
use iced::advanced::Widget;
use rodio::{OutputStream, Sink};

use crate::commands;

mod icons;

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

        let main_column = column![
            row![
                text("DapEq").size(20),
            ]
                .padding([14,0,14,0]),
            row![
                icons::play_action(icons::backward_button(), Control::Backward),
                icons::play_action(flip_button, Control::PlayPause),
                icons::play_action(icons::forward_button(), Control::Forward),
            ]
                .align_items(iced::Alignment::Center)
                .spacing(4),
            column![
                icons::control_action(icons::go_back_button(), Control::GoBack),
            ]
                .align_items(iced::Alignment::Start)
                // Top, Right, Bottom, Left
                .padding([22,100,0,0]),
        ]
            .align_items(iced::Alignment::Center);

        main_column.into()
    }
}
