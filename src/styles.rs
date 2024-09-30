use iced::{
    widget::{button, container},
    Border, Theme,
};

pub fn pane_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.weak.color.into()),
        border: Border {
            width: 2.0,
            color: palette.background.strong.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn selected_project(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    button::Style {
        background: Some(palette.background.strong.color.into()),
        text_color: palette.primary.strong.color,
        ..Default::default()
    }
}

pub fn project(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    button::Style {
        background: Some(palette.background.weak.color.into()),
        text_color: palette.primary.strong.color,
        ..Default::default()
    }
}

pub fn selected_service(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    button::Style {
        background: Some(palette.background.strong.color.into()),
        text_color: palette.primary.strong.color,
        ..Default::default()
    }
}

pub fn service(theme: &Theme, _status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    button::Style {
        background: Some(palette.background.weak.color.into()),
        text_color: palette.primary.strong.color,
        ..Default::default()
    }
}
