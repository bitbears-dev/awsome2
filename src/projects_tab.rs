use iced::{
    widget::{container, pane_grid, scrollable::AbsoluteOffset, text, PaneGrid},
    Element, Length, Task,
};
use iced_aw::widgets::SelectionList;

use crate::{
    fonts::get_default_font,
    message::Message,
    resource_details::ResourceDetails,
    resources_table::ResourcesTable,
    state::State,
    styles,
    workspace::{Project, Resource},
};

#[derive(Clone, Debug)]
struct Pane {
    id: PaneId,
}

#[derive(Clone, Debug)]
enum PaneId {
    ProjectSelector,
    ResourceTable,
    ResourceDetails,
}

impl std::fmt::Display for PaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneId::ProjectSelector => write!(f, "Project Selector"),
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
    projects: Vec<Project>,
    selected_project: Option<Project>,
    _selected_resource: Option<Resource>,
    resources_table: ResourcesTable,
    resource_details: ResourceDetails,
}

impl ProjectsTab {
    pub fn new() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.15,
            a: Box::new(pane_grid::Configuration::Pane(Pane::new(
                PaneId::ProjectSelector,
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
            projects: Vec::new(),
            selected_project: None,
            _selected_resource: None,
            resources_table: ResourcesTable::new(),
            resource_details: ResourceDetails::new(),
        }
    }

    pub fn set_selected_project(&mut self, project: Option<Project>) {
        self.selected_project = project.clone();
        self.resources_table.set_selected_project(project);
    }

    //pub fn set_selected_resource(&mut self, resource: Option<Resource>) {
    //    self.resource_details.set_selected_resource(resource);
    //}

    pub fn sync_resources_table_header_offset(&mut self, offset: AbsoluteOffset) -> Task<Message> {
        self.resources_table.sync_header_offset(offset)
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        let pane_grid = PaneGrid::new(&self.panes, |_pane_number, pane, _is_maximized| {
            let title_bar = pane_grid::TitleBar::new(text(pane.id.to_string()));

            pane_grid::Content::new(self.view_content(pane, state))
                .title_bar(title_bar)
                .style(styles::pane_active)
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

    fn view_content<'a>(&'a self, pane: &Pane, state: &'a State) -> Element<'a, Message> {
        match pane.id {
            PaneId::ProjectSelector => self.render_project_selector(state),
            PaneId::ResourceTable => self.render_resource_table(),
            PaneId::ResourceDetails => self.resource_details.view(),
        }
    }

    fn render_project_selector<'a>(&self, state: &'a State) -> Element<'a, Message> {
        let project_index = self
            .projects
            .iter()
            .position(|project| Some(project) == self.selected_project.as_ref());

        let project_selector = SelectionList::new_with(
            &state.workspace.projects,
            Message::ProjectSelected,
            14.0,
            5.0,
            iced_aw::style::selection_list::primary,
            project_index,
            get_default_font(),
        )
        .width(Length::Fill)
        .height(Length::Fill);
        container(project_selector)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn render_resource_table(&self) -> Element<Message> {
        match &self.selected_project {
            Some(_) => self.resources_table.view(),
            None => container(text("Select a project to view resources"))
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into(),
        }
    }
}
