use std::path::PathBuf;

use clap::Parser;
use iced::{
    widget::{container, text},
    Application, Command,
};

use crate::{
    main_tab::MainTab, message::Message, pane_type::PaneType, regions::load_regions,
    resource::load_resources, state::State, workspace,
};

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
            Message::ProfileSelected(profile) => {
                self.main_tab
                    .explore_tab
                    .set_selected_profile(profile.clone());
                let nearest_region = self
                    .state
                    .as_ref()
                    .map(|s| s.get_nearest_region().to_string())
                    .unwrap_or("us-east-1".to_string());
                iced::Command::perform(load_regions(profile.clone(), nearest_region), |res| {
                    match res {
                        Ok(regions) => Message::RegionsLoaded(regions),
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            Message::RegionsLoaded(vec![])
                        }
                    }
                })
            }
            Message::RegionsLoaded(regions) => {
                self.main_tab.explore_tab.set_regions(regions);
                iced::Command::none()
            }
            Message::RegionSelected(region) => {
                self.main_tab.explore_tab.set_selected_region(region);
                iced::Command::none()
            }
            Message::ServiceSelected(_index, service) => {
                self.main_tab.explore_tab.set_selected_service(service);
                self.main_tab.explore_tab.set_resources(vec![]);

                let Some(profile) = self.main_tab.explore_tab.get_selected_profile() else {
                    return iced::Command::none();
                };
                let Some(region) = self.main_tab.explore_tab.get_selected_region() else {
                    return iced::Command::none();
                };

                self.main_tab.explore_tab.set_loading_resources(true);
                iced::Command::perform(load_resources(profile, region, service), |res| match res {
                    Ok(resources) => Message::ResourcesLoaded(resources),
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                        Message::ResourcesLoaded(vec![])
                    }
                })
            }
            Message::ResourcesLoaded(resources) => {
                self.main_tab.explore_tab.set_loading_resources(false);
                self.main_tab.explore_tab.set_resources(resources);
                iced::Command::none()
            }
            Message::ResourceSelected(_index, resource) => {
                self.main_tab.explore_tab.set_selected_resource(resource);
                iced::Command::none()
            }
            Message::Splitter1Event(event) => {
                let msg = self.main_tab.explore_tab.splitter1.update(event);
                if let Some(msg) = msg {
                    return self.update(msg);
                }
                iced::Command::none()
            }
            Message::Splitter1Moved(pos) => {
                self.main_tab.explore_tab.set_service_selector_width(pos);
                iced::Command::none()
            }
            Message::Splitter2Event(event) => {
                let msg = self.main_tab.explore_tab.splitter2.update(event);
                if let Some(msg) = msg {
                    return self.update(msg);
                }
                iced::Command::none()
            }
            Message::Splitter2Moved(pos) => {
                self.main_tab.explore_tab.set_resource_selector_width(pos);
                iced::Command::none()
            }
            Message::DoNothing => iced::Command::none(),
            Message::DoNothingOnToggle(_) => iced::Command::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        match &self.state {
            None => self.loading_view(),
            Some(state) => self.main_tab.view(state),
        }
    }
}
