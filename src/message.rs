use crate::{error::Error, resource::Resource, service::Service, state::State};

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    WorkspaceLoaded(Result<State, Error>),
    SideDrawerToggled,
    ActivateExploreTab,
    ActivateProjectsTab,

    ProfileSelected(String),
    RegionsLoaded(Vec<String>),
    RegionSelected(String),
    ServiceSelected(usize, &'static Service),
    ResourcesLoaded(Vec<Resource>),
    ResourceSelected(usize, Resource),

    ExploreTabPaneResized(iced::widget::pane_grid::ResizeEvent),

    DoNothing,
    #[allow(dead_code)]
    DoNothingOnToggle(bool),
}
