use std::path::PathBuf;

use clap::Parser;
use iced::{
    widget::{container, text},
    Task,
};

use crate::{
    log_receiver, main_tab::MainTab, message::Message, pane_type::PaneType,
    region::load_region_names, resource::load_resources, state::State, workspace::Workspace,
};

#[derive(Default, Parser)]
pub struct AppFlags {
    #[clap(short, long)]
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
                    Workspace::load(flags.workspace_file),
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
                        eprintln!("Error while loading font: {:?}", e);
                    }
                }
                Task::none()
            }
            Message::WorkspaceLoaded(workspace) => {
                let workspace = match workspace {
                    Ok(workspace) => workspace,
                    Err(e) => {
                        eprintln!("Error while loading workspace: {:?}", e);
                        self.state = Some(State::new());
                        return Task::done(Message::ErrorOccurred(e));
                    }
                };

                let state = match State::from_workspace(workspace) {
                    Ok(state) => state,
                    Err(e) => {
                        eprintln!("Error while loading state: {:?}", e);
                        self.state = Some(State::new());
                        return Task::done(Message::ErrorOccurred(e));
                    }
                };

                self.main_tab
                    .projects_tab
                    .set_projects(state.workspace.projects.clone());
                self.state = Some(state);
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
                    load_region_names(profile.clone(), nearest_region),
                    |res| match res {
                        Ok(regions) => Message::RegionsLoaded(regions),
                        Err(e) => {
                            eprintln!("Error while loading regions: {:?}", e);
                            Message::ErrorOccurred(e)
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
                        eprintln!("Error while loading resource: {:?}", e);
                        Message::ErrorOccurred(e)
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
            Message::ProjectSelected(_index, project) => {
                self.main_tab
                    .projects_tab
                    .project_service_selector
                    .toggle_project(&project);
                self.main_tab
                    .projects_tab
                    .project_service_selector
                    .set_selected_project(Some(project.clone()));
                Task::none()
            }
            Message::ProjectServiceSelected(_index, project, service) => {
                self.main_tab
                    .projects_tab
                    .project_service_selector
                    .set_selected_service(Some(project.clone()), Some(service.clone()));
                self.main_tab
                    .projects_tab
                    .resources_table
                    .set_selected_project_and_service(Some(project), Some(service));
                Task::none()
            }
            //Message::ProjectResourceSelected(_index, resource) => {
            //    self.main_tab
            //        .projects_tab
            //        .set_selected_resource(Some(resource));
            //    Task::none()
            //}
            Message::SyncResourcesTableHeader(offset) => self
                .main_tab
                .projects_tab
                .sync_resources_table_header_offset(offset),
            Message::ErrorOccurred(e) => {
                let Some(state) = &mut self.state else {
                    return Task::none();
                };
                state.append_log(format!("Error: {:?}", e));
                Task::none()
            }

            Message::DoNothing => Task::none(),
            Message::DoNothingOnToggle(_) => Task::none(),
            Message::LogReceiverReady(sender) => {
                if let Some(state) = &mut self.state {
                    state.set_log_sender(sender);
                }
                Task::none()
            }
            Message::LogReceived(log) => {
                if let Some(state) = &mut self.state {
                    state.append_log(log);
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> iced::Element<Message> {
        match &self.state {
            None => self.loading_view(),
            Some(state) => self.main_tab.view(state),
        }
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::run(log_receiver::start)
    }
}
