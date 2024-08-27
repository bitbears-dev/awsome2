mod app;
mod bootstrap_text;
mod easing;
mod error;
mod explore_tab;
mod fonts;
mod lambda_function_details;
mod linear;
mod main_tab;
mod message;
mod pane_type;
mod profiles;
mod regions;
mod resource;
mod service;
mod state;
mod workspace;

use crate::app::AwsomeApp;

fn main() -> iced::Result {
    iced::application("Awsome", AwsomeApp::update, AwsomeApp::view)
        .settings(iced::Settings {
            default_font: iced::Font {
                family: iced::font::Family::Name("Noto Sans Mono"),
                ..Default::default()
            },
            default_text_size: iced::Pixels(14.0),
            antialiasing: true,
            ..iced::Settings::default()
        })
        .run_with(AwsomeApp::new)
}
