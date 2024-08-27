use std::path::PathBuf;

use clap::Parser;
use iced::{
    widget::{container, text},
    Task,
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

impl Default for AwsomeApp {
    fn default() -> Self {
        AwsomeApp {
            state: None,
            main_tab: MainTab::new(),
        }
    }
}

impl AwsomeApp {
    fn loading_view(&self) -> iced::Element<Message> {
        container(
            text("Loading...")
                .align_x(iced::alignment::Horizontal::Center)
                .size(50),
        )
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .center(iced::Length::Fill)
        .into()
    }

    pub fn new() -> (Self, Task<Message>) {
        let flags = AppFlags::parse();

        (
            AwsomeApp {
                state: None,
                main_tab: MainTab::new(),
            },
            Task::batch([
                iced::font::load(include_bytes!(concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/assets/fonts/icons.ttf"
                )))
                .map(Message::FontLoaded),
                Task::perform(
                    workspace::load(flags.workspace_file),
                    Message::WorkspaceLoaded,
                ),
            ]),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::FontLoaded(result) => {
                match result {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {:?}", e);
                    }
                }
                Task::none()
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
                Task::none()
            }
            Message::SideDrawerToggled => {
                if let Some(state) = &mut self.state {
                    state.toggle_side_drawer();
                }
                Task::none()
            }
            Message::ActivateExploreTab => {
                if let Some(state) = &mut self.state {
                    state.set_active_pane(PaneType::Explore);
                    state.close_side_drawer();
                }
                Task::none()
            }
            Message::ActivateProjectsTab => {
                if let Some(state) = &mut self.state {
                    state.set_active_pane(PaneType::Projects);
                    state.close_side_drawer();
                }
                Task::none()
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
                Task::perform(
                    load_regions(profile.clone(), nearest_region),
                    |res| match res {
                        Ok(regions) => Message::RegionsLoaded(regions),
                        Err(e) => {
                            eprintln!("Error: {:?}", e);
                            Message::RegionsLoaded(vec![])
                        }
                    },
                )
            }
            Message::RegionsLoaded(regions) => {
                self.main_tab.explore_tab.set_regions(regions);
                Task::none()
            }
            Message::RegionSelected(region) => {
                self.main_tab.explore_tab.set_selected_region(region);
                Task::none()
            }
            Message::ServiceSelected(_index, service) => {
                self.main_tab.explore_tab.set_selected_service(service);
                self.main_tab.explore_tab.set_resources(vec![]);

                let Some(profile) = self.main_tab.explore_tab.get_selected_profile() else {
                    return Task::none();
                };
                let Some(region) = self.main_tab.explore_tab.get_selected_region() else {
                    return Task::none();
                };

                self.main_tab.explore_tab.set_loading_resources(true);
                Task::perform(load_resources(profile, region, service), |res| match res {
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
                Task::none()
            }
            Message::ResourceSelected(_index, resource) => {
                self.main_tab.explore_tab.set_selected_resource(resource);
                Task::none()
            }
            Message::ExploreTabPaneResized(event) => {
                self.main_tab.explore_tab.resize_pane(event);
                Task::none()
            }
            Message::DoNothing => Task::none(),
            Message::DoNothingOnToggle(_) => Task::none(),
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.state {
            None => self.loading_view(),
            Some(state) => self.main_tab.view(state),
        }
    }
}
