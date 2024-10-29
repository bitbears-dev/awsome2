use iced::Font;

pub fn get_default_font() -> Font {
    Font {
        family: iced::font::Family::Name("Noto Sans Mono"),
        weight: iced::font::Weight::Normal,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}

pub fn get_bold_font() -> Font {
    Font {
        family: iced::font::Family::Name("Noto Sans Mono"),
        weight: iced::font::Weight::Bold,
        stretch: iced::font::Stretch::Normal,
        style: iced::font::Style::Normal,
    }
}
