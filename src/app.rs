use std::path::PathBuf;

use clap::Parser;
use iced::{
    widget::{container, text},
    Application, Command,
};

use crate::{main_tab::MainTab, message::Message, pane_type::PaneType, state::State, workspace};

#[derive(Default, Parser)]
pub struct AppFlags {
    workspace_file: Option<PathBuf>,
}

pub struct AwsomeApp {
    state: Option<State>,
    main_tab: MainTab,
}

impl AwsomeApp {
    fn loading_view(&self) -> iced::Element<Message> {
        container(
            text("Loading...")
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .size(50),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
}

impl Application for AwsomeApp {
    type Message = Message;
    type Theme = iced::Theme;
    type Executor = iced::executor::Default;
    type Flags = AppFlags;

    fn new(flags: Self::Flags) -> (AwsomeApp, iced::Command<Message>) {
        (
            AwsomeApp {
                state: None,
                main_tab: MainTab::new(),
            },
            Command::batch([
                iced::font::load(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/fonts/icons.ttf"
                )))
                .map(Message::FontLoaded),
                Command::perform(
                    workspace::load(flags.workspace_file),
                    Message::WorkspaceLoaded,
                ),
            ]),
        )
    }

    fn title(&self) -> String {
        String::from("Awsome App")
    }

    fn update(&mut self, message: Message) -> iced::Command<Message> {
        match message {
            Message::FontLoaded(result) => {
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
                iced::Command::none()
            }
            Message::WorkspaceLoaded(state) => {
                match state {
                    Ok(state) => {
                        self.state = Some(state);
                    }
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        self.state = Some(State::new());
                    }
                }
                iced::Command::none()
            }
            Message::SideDrawerToggled => {
                if let Some(state) = &mut self.state {
                    state.toggle_side_drawer();
                }
                iced::Command::none()
            }
            Message::ActivateExploreTab => {
                if let Some(state) = &mut self.state {
                    state.set_active_pane(PaneType::Explore);
                    state.close_side_drawer();
                }
                iced::Command::none()
            }
            Message::ActivateProjectsTab => {
                if let Some(state) = &mut self.state {
                    state.set_active_pane(PaneType::Projects);
                    state.close_side_drawer();
                }
                iced::Command::none()
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.state {
            None => self.loading_view(),
            Some(state) => self.main_tab.view(state),
        }
    }
}
