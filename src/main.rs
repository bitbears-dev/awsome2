mod app;
mod bootstrap_text;
mod error;
mod main_tab;
mod message;
mod pane_type;
mod state;
mod workspace;

use clap::Parser;
use iced::Application;

use crate::app::{AppFlags, AwsomeApp};

fn main() -> iced::Result {
    let flags = AppFlags::parse();

    AwsomeApp::run(iced::Settings {
        flags,
        default_font: iced::Font {
            family: iced::font::Family::Name("Noto Sans Mono"),
            ..Default::default()
        },
        default_text_size: iced::Pixels(14.0),
        ..iced::Settings::default()
    })
}
