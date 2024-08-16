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

    Splitter1Event(crate::splitter::Event),
    Splitter1Moved(f32),

    Splitter2Event(crate::splitter::Event),
    Splitter2Moved(f32),

    DoNothing,
    #[allow(dead_code)]
    DoNothingOnToggle(bool),
}
