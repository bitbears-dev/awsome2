mod app;
mod error;
//mod log_receiver;
mod message;
mod models;
mod state;
mod view;

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
        //.subscription(AwsomeApp::subscription)
        .run_with(AwsomeApp::new)
}
