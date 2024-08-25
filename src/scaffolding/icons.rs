use iced::{Element, Font};
use iced::{widget::{button, text, container}};
use scaffolding::Control;
use crate::scaffolding;

pub fn play_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E800}')
}
pub fn pause_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E801}')
}
pub fn forward_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E802}')
}
pub fn backward_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E803}')
}
pub fn go_back_button<'a, Message>() -> Element<'a, Message> {
    icon('\u{E804}')
}

fn icon<'a, Message>(codepoint: char) -> Element<'a, Message> {
    const ICON: Font = Font::with_name("play_button");
    text(codepoint).font(ICON).into()
}

pub fn play_action(content: Element<Control>, on_press: Control) -> Element<Control> {
    button(container(content).width(20).center_x())
        .on_press(on_press)
        .padding([5, 10])
        .into()
}

pub fn control_action(content: Element<Control>, on_press: Control) -> Element<Control> {
    button(container(content).width(20).center_x())
        .on_press(on_press)
        .into()
}
