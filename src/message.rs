use crate::{
    error::Error,
    resource::Resource,
    service::Service,
    workspace::{Project, ResourceDescriptor, Workspace},
};

#[derive(Debug, Clone)]
pub enum Message {
    FontLoaded(Result<(), iced::font::Error>),
    WorkspaceLoaded(Result<Workspace, Error>),
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

    ProjectSelected(usize, Project),
    ProjectServiceSelected(usize, Project, Service),
    //ProjectResourceSelected(usize, Resource),
    SyncResourcesTableHeader(iced::widget::scrollable::AbsoluteOffset),
    ResourcesTableColumnResizing(usize, f32),
    ResourcesTableColumnResized,
    ResourcesTableCellClicked(usize, usize, ResourceDescriptor),
    ResourceDetailsLoaded(Resource),

    ErrorOccurred(Error),

    //    LogReceiverReady(iced::futures::channel::mpsc::Sender<String>),
    //    LogReceived(String),
    DoNothing,
    #[allow(dead_code)]
    DoNothingOnToggle(bool),
}
