use iced::{
    widget::{button, container},
    Border, Theme,
};

pub fn header(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            color: palette.background.weak.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn menu_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.weak.color.into()),
            text_color: palette.primary.weak.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.primary.base.color.into()),
            text_color: palette.primary.base.text,
            ..Default::default()
        },
    }
}

pub fn tool_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.secondary.strong.color.into()),
            text_color: palette.secondary.strong.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.secondary.base.color.into()),
            text_color: palette.secondary.base.text,
            ..Default::default()
        },
    }
}

pub fn pane_active(theme: &Theme) -> container::Style {
    let palette = theme.extended_palette();

    container::Style {
        background: Some(palette.background.base.color.into()),
        border: Border {
            width: 1.0,
            color: palette.background.weak.color,
            ..Border::default()
        },
        ..Default::default()
    }
}

pub fn side_drawer_button_selected(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.weak.color.into()),
            text_color: palette.primary.weak.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.primary.strong.color.into()),
            text_color: palette.primary.strong.text,
            ..Default::default()
        },
    }
}

pub fn side_drawer_button(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.background.strong.color.into()),
            text_color: palette.primary.strong.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.background.weak.color.into()),
            text_color: palette.primary.weak.text,
            ..Default::default()
        },
    }
}

pub fn selected_project(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.strong.color.into()),
            text_color: palette.primary.strong.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.primary.base.color.into()),
            text_color: palette.primary.base.text,
            ..Default::default()
        },
    }
}

pub fn project(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.background.weak.color.into()),
            text_color: palette.background.weak.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.background.base.color.into()),
            text_color: palette.background.base.text,
            ..Default::default()
        },
    }
}

pub fn selected_service(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.primary.strong.color.into()),
            text_color: palette.primary.strong.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.primary.base.color.into()),
            text_color: palette.primary.base.text,
            ..Default::default()
        },
    }
}

pub fn service(theme: &Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    match status {
        button::Status::Hovered => button::Style {
            background: Some(palette.background.weak.color.into()),
            text_color: palette.background.weak.text,
            ..Default::default()
        },
        _ => button::Style {
            background: Some(palette.background.base.color.into()),
            text_color: palette.background.base.text,
            ..Default::default()
        },
    }
}

pub fn service_selection_list(
    theme: &Theme,
    _status: iced_aw::style::status::Status,
) -> iced_aw::style::selection_list::Style {
    let palette = theme.extended_palette();

    iced_aw::style::selection_list::Style {
        background: palette.background.base.color.into(),
        ..Default::default()
    }
}

pub fn resource_selection_list(
    theme: &Theme,
    _status: iced_aw::style::status::Status,
) -> iced_aw::style::selection_list::Style {
    let palette = theme.extended_palette();

    iced_aw::style::selection_list::Style {
        background: palette.background.base.color.into(),
        ..Default::default()
    }
}
