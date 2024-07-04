use iced::{
    widget::{text, Text},
    Font,
};

pub fn bootstrap_text(t: &str, size: f32, width: iced::Length, height: iced::Length) -> Text {
    text(t)
        .font(Font::with_name("bootstrap-icons"))
        .size(size)
        .width(width)
        .height(height)
        .horizontal_alignment(iced::alignment::Horizontal::Center)
        .vertical_alignment(iced::alignment::Vertical::Center)
}
