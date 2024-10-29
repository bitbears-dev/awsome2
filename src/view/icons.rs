use iced::Element;
use lazy_static::lazy_static;

use crate::{message::Message, view::bootstrap_text::bootstrap_text};

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

    pub fn compass(
        &self,
        size: f32,
        width: iced::Length,
        height: iced::Length,
    ) -> Element<Message> {
        bootstrap_text("\u{F2D1}", size, width, height).into()
    }

    pub fn folder_plus(
        &self,
        size: f32,
        width: iced::Length,
        height: iced::Length,
    ) -> Element<Message> {
        bootstrap_text("\u{F3D3}", size, width, height).into()
    }

    pub fn list(&self, size: f32, width: iced::Length, height: iced::Length) -> Element<Message> {
        bootstrap_text("\u{F479}", size, width, height).into()
    }
}
