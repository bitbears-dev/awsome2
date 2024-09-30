use iced::Element;
use lazy_static::lazy_static;

use crate::{bootstrap_text::bootstrap_text, message::Message};

lazy_static! {
    pub static ref ICONS: Icons = Icons::new();
}

pub struct Icons {}

impl Icons {
    pub fn new() -> Self {
        Self {}
    }

    pub fn chevron_down(
        &self,
        size: f32,
        width: iced::Length,
        height: iced::Length,
    ) -> Element<Message> {
        bootstrap_text("\u{F282}", size, width, height).into()
    }

    pub fn chevron_right(
        &self,
        size: f32,
        width: iced::Length,
        height: iced::Length,
    ) -> Element<Message> {
        bootstrap_text("\u{F285}", size, width, height).into()
    }
}
