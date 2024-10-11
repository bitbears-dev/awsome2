mod app;
mod bootstrap_text;
mod easing;
mod error;
mod explore_tab;
mod fonts;
mod footer;
mod icons;
mod linear;
//mod log_receiver;
mod main_tab;
mod message;
mod pane_type;
mod profile;
mod project_service_selector;
mod projects_tab;
mod region;
mod resource;
mod resource_details;
mod resources_table;
mod service;
mod state;
mod styles;
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
        //.subscription(AwsomeApp::subscription)
        .run_with(AwsomeApp::new)
}
