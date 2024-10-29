mod project_service_selector;

use iced::{
    widget::{container, pane_grid, scrollable::AbsoluteOffset, PaneGrid},
    Element, Length, Task,
};

use crate::{
    message::Message,
    models::workspace::Project,
    view::{
        projects_tab::project_service_selector::ProjectServiceSelector,
        resource_details::ResourceDetails, resources_table::ResourcesTable, styles,
    },
};

#[derive(Clone, Debug)]
struct Pane {
    id: PaneId,
}

#[derive(Clone, Debug)]
enum PaneId {
    ProjectServiceSelector,
    ResourceTable,
    ResourceDetails,
}

impl std::fmt::Display for PaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneId::ProjectServiceSelector => write!(f, "Projects"),
            PaneId::ResourceTable => write!(f, "Resource Table"),
            PaneId::ResourceDetails => write!(f, "Resource Details"),
        }
    }
}

impl Pane {
    pub fn new(id: PaneId) -> Self {
        Self { id }
    }
}

pub struct ProjectsTab {
    panes: pane_grid::State<Pane>,
    _projects: Vec<Project>,
    pub project_service_selector: ProjectServiceSelector,
    pub resources_table: ResourcesTable,
    pub resource_details: ResourceDetails,
}

impl ProjectsTab {
    pub fn new() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.3,
            a: Box::new(pane_grid::Configuration::Pane(Pane::new(
                PaneId::ProjectServiceSelector,
            ))),
            b: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Horizontal,
                ratio: 0.2,
                a: Box::new(pane_grid::Configuration::Pane(Pane::new(
                    PaneId::ResourceTable,
                ))),
                b: Box::new(pane_grid::Configuration::Pane(Pane::new(
                    PaneId::ResourceDetails,
                ))),
            }),
        };
        let panes = pane_grid::State::with_configuration(config);

        Self {
            panes,
            _projects: Vec::new(),
            project_service_selector: ProjectServiceSelector::new(),
            resources_table: ResourcesTable::new(),
            resource_details: ResourceDetails::new(),
        }
    }

    pub fn set_projects(&mut self, projects: Vec<Project>) {
        self._projects = projects.clone();
        self.project_service_selector.set_projects(&projects);
    }

    pub fn sync_resources_table_header_offset(&mut self, offset: AbsoluteOffset) -> Task<Message> {
        self.resources_table.sync_header_offset(offset)
    }

    pub fn view(&self) -> Element<Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane_number, pane, _is_maximized| {
            pane_grid::Content::new(self.view_content(pane)).style(styles::pane_active)
        })
        .on_resize(10, Message::ExploreTabPaneResized)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

        container(pane_grid)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_content(&self, pane: &Pane) -> Element<Message> {
        match pane.id {
            PaneId::ProjectServiceSelector => self.project_service_selector.view(),
            PaneId::ResourceTable => self.resources_table.view(),
            PaneId::ResourceDetails => self.resource_details.view(),
        }
    }
}
