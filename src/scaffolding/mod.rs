use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;
use std::thread;
use iced::{Application, Command, Element, Font};
use iced::{widget::{column, row, button}};
use iced::mouse::Button;
use iced::widget::text;
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

// ICONS
fn play_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E800}')
}
fn pause_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E801}')
}
fn forward_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E802}')
}
fn backward_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E803}')
}
fn go_back_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E804}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON: Font = Font::with_name("play_button");
    text(codepoint).font(ICON).into()
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

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let flip_button = match self.state {
            State::Paused =>
                play_button(),
            State::Playing =>
                pause_button(),

        };
        
        column![
            button(go_back_button()).on_press(Self::Message::GoBack),
            row![
                button(backward_button()).on_press(Self::Message::Backward),
                button(flip_button).on_press(Self::Message::PlayPause),
                button(forward_button()).on_press(Self::Message::Forward),
            ]
        ].into()
    }
}

