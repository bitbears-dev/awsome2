use iced::{
    alignment,
    widget::{button, column, container, row, text},
    Element, Length, Padding,
};

use crate::{
    message::Message,
    models::{service::Service, workspace::Project},
    view::{icons::ICONS, styles},
};

struct ProjectItem {
    index: usize,
    project: Project,
    _selected: bool,
    open: bool,
}

impl ProjectItem {
    pub fn new(index: usize, project: Project) -> Self {
        Self {
            index,
            project,
            _selected: false,
            open: false,
        }
    }

    pub fn view(
        &self,
        selected_project: &Option<Project>,
        selected_service: &Option<Service>,
    ) -> Element<Message> {
        let mut c = column![].height(Length::Shrink);
        let toggle_mark = if self.open {
            ICONS.chevron_down(12.0, Length::Fixed(16.0), Length::Fixed(16.0))
        } else {
            ICONS.chevron_right(12.0, Length::Fixed(16.0), Length::Fixed(16.0))
        };
        let is_selected_project = selected_project
            .clone()
            .filter(|p| p == &self.project)
            .is_some();
        let project_style = if is_selected_project {
            styles::selected_project
        } else {
            styles::project
        };

        c = c.push(
            button(
                row![toggle_mark, text(&self.project.name)]
                    .width(Length::Fill)
                    .align_y(alignment::Alignment::Center),
            )
            .on_press(Message::ProjectSelected(self.index, self.project.clone()))
            .style(project_style),
        );
        if self.open {
            for service in self.project.get_services().iter() {
                let is_selected_service =
                    selected_service.clone().filter(|s| s == service).is_some();
                let service_style = if is_selected_project && is_selected_service {
                    styles::selected_service
                } else {
                    styles::service
                };
                c = c.push(
                    row![
                        text(" ")
                            .width(Length::Fixed(16.0))
                            .height(Length::Fixed(16.0)),
                        button(text(format!("  {}", service)))
                            .on_press(Message::ProjectServiceSelected(
                                self.index,
                                self.project.clone(),
                                service.clone(),
                            ))
                            .width(Length::Fill)
                            .style(service_style),
                    ]
                    .height(Length::Shrink),
                );
            }
        }

        container(c)
            .padding(Padding::from(2.0))
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }
}

pub struct ProjectServiceSelector {
    project_items: Vec<ProjectItem>,
    selected_project: Option<Project>,
    selected_service: Option<Service>,
}

impl ProjectServiceSelector {
    pub fn new() -> Self {
        Self {
            project_items: vec![],
            selected_project: None,
            selected_service: None,
        }
    }

    pub fn set_projects(&mut self, projects: &[Project]) {
        self.project_items = projects
            .iter()
            .enumerate()
            .map(|(index, p)| ProjectItem::new(index, p.clone()))
            .collect();
    }

    pub fn toggle_project(&mut self, project: &Project) {
        if let Some(pi) = self
            .project_items
            .iter_mut()
            .find(|pi| &pi.project == project)
        {
            pi.open = !pi.open;
        }
    }

    pub fn set_selected_project(&mut self, project: Option<Project>) {
        if self.selected_project != project {
            self.selected_service = None;
        }
        self.selected_project = project;
    }

    pub fn set_selected_service(&mut self, project: Option<Project>, service: Option<Service>) {
        self.selected_project = project;
        self.selected_service = service;
    }

    pub fn view(&self) -> Element<Message> {
        let buttons = self.render_buttons();
        let mut c = column![buttons];

        for p in self.project_items.iter() {
            c = c.push(p.view(&self.selected_project, &self.selected_service));
        }

        container(c)
            .width(Length::Fill)
            .height(Length::Shrink)
            .into()
    }

    pub fn render_buttons(&self) -> Element<Message> {
        let project_add_icon = ICONS.folder_plus(10.0, Length::Fixed(16.0), Length::Fixed(16.0));
        let project_add_button = button(project_add_icon)
            .on_press(Message::AddProject)
            .style(styles::tool_button)
            .width(iced::Length::Shrink);

        let buttons = row![project_add_button,];

        container(buttons)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .into()
    }
}
