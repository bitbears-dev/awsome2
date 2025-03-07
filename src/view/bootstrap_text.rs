use iced::{
    widget::{text, Text},
    Font,
};

// Gallery: https://icons.getbootstrap.com/

pub fn bootstrap_text(t: &str, size: f32, width: iced::Length, height: iced::Length) -> Text {
    text(t)
        .font(Font::with_name("bootstrap-icons"))
        .size(size)
        .width(width)
        .height(height)
        .align_x(iced::alignment::Horizontal::Center)
        .align_y(iced::alignment::Vertical::Center)
}
