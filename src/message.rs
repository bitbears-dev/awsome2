use crate::{error::Error, state::State};

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
}
