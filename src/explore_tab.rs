use std::time::Duration;

use iced::{
    widget::{
        column, container,
        pane_grid::{self, PaneGrid},
        pick_list,
    },
    Element, Length,
};
use iced_aw::SelectionList;

use crate::{
    easing, fonts::get_default_font, linear::Linear, message::Message, profile::load_profile_names,
    resource::Resource, resource_details::ResourceDetails, service::Service, styles,
};

#[derive(Clone, Debug)]
struct Pane {
    id: PaneId,
}

#[derive(Clone, Debug)]
enum PaneId {
    ServiceSelector,
    ResourceSelector,
    ResourceDetails,
}

impl std::fmt::Display for PaneId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PaneId::ServiceSelector => write!(f, "Service Selector"),
            PaneId::ResourceSelector => write!(f, "Resource Selector"),
            PaneId::ResourceDetails => write!(f, "Resource Details"),
        }
    }
}

impl Pane {
    pub fn new(id: PaneId) -> Self {
        Self { id }
    }
}

const SERVICES: &[&Service] = &[&Service::Lambda, &Service::S3];

pub struct ExploreTab {
    panes: pane_grid::State<Pane>,
    profiles: Vec<String>,
    selected_profile: Option<String>,
    regions: Vec<String>,
    selected_region: Option<String>,
    services: Vec<&'static Service>,
    selected_service: Option<&'static Service>,
    loading_resources: bool,
    resources: Vec<Resource>,
    //selected_resource: Option<Resource>,
    resource_details: ResourceDetails,
}

impl ExploreTab {
    pub fn new() -> Self {
        let config = pane_grid::Configuration::Split {
            axis: pane_grid::Axis::Vertical,
            ratio: 0.15,
            a: Box::new(pane_grid::Configuration::Pane(Pane::new(
                PaneId::ServiceSelector,
            ))),
            b: Box::new(pane_grid::Configuration::Split {
                axis: pane_grid::Axis::Vertical,
                ratio: 0.2,
                a: Box::new(pane_grid::Configuration::Pane(Pane::new(
                    PaneId::ResourceSelector,
                ))),
                b: Box::new(pane_grid::Configuration::Pane(Pane::new(
                    PaneId::ResourceDetails,
                ))),
            }),
        };
        let panes = pane_grid::State::with_configuration(config);
        let profiles = load_profile_names().unwrap_or_else(|err| {
            eprintln!("Failed to load profiles: {:?}", err);
            vec![]
        });

        Self {
            panes,
            profiles,
            selected_profile: None,
            regions: vec![],
            selected_region: None,
            services: SERVICES.to_vec(),
            selected_service: None,
            loading_resources: false,
            resources: vec![],
            //selected_resource: None,
            resource_details: ResourceDetails::new(),
        }
    }

    pub fn set_selected_profile(&mut self, profile: String) {
        self.selected_profile = Some(profile);
    }

    pub fn get_selected_profile(&self) -> Option<String> {
        self.selected_profile.clone()
    }

    pub fn set_regions(&mut self, regions: Vec<String>) {
        self.regions = regions;
    }

    pub fn set_selected_region(&mut self, region: String) {
        self.selected_region = Some(region);
    }

    pub fn get_selected_region(&self) -> Option<String> {
        self.selected_region.clone()
    }

    pub fn set_selected_service(&mut self, service: &'static Service) {
        self.selected_service = Some(service);
    }

    pub fn set_loading_resources(&mut self, loading: bool) {
        self.loading_resources = loading;
    }

    pub fn set_resources(&mut self, resources: Vec<Resource>) {
        self.resources = resources;
    }

    pub fn set_selected_resource(&mut self, resource: Resource) {
        self.resource_details.set_resource(Some(resource));
    }

    pub fn resize_pane(&mut self, event: iced::widget::pane_grid::ResizeEvent) {
        self.panes.resize(event.split, event.ratio);
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
            PaneId::ServiceSelector => self.render_service_selector(),
            PaneId::ResourceSelector => self.render_resource_selector(),
            PaneId::ResourceDetails => self.resource_details.view(),
        }
    }

    fn render_service_selector(&self) -> Element<Message> {
        let profile_selector = pick_list(
            self.profiles.clone(),
            self.selected_profile.clone(),
            Message::ProfileSelected,
        )
        .placeholder("Profile")
        .width(Length::Fill);
        let mut c = column![profile_selector]
            .width(iced::Length::Fill)
            .height(Length::Fill);

        if self.selected_profile.is_some() {
            let region_selector = pick_list(
                self.regions.clone(),
                self.selected_region.clone(),
                Message::RegionSelected,
            )
            .placeholder("Region")
            .width(Length::Fill);
            c = c.push(region_selector);
        }

        if self.selected_region.is_some() {
            let service_selector = SelectionList::new_with(
                &self.services,
                Message::ServiceSelected,
                14.0,
                5.0,
                iced_aw::style::selection_list::primary,
                None,
                get_default_font(),
            )
            .style(styles::service_selection_list)
            .width(Length::Fill)
            .height(Length::Fill);
            c = c.push(service_selector);
        }
        container(c)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn render_resource_selector(&self) -> Element<Message> {
        if self.loading_resources {
            return container(
                Linear::new()
                    .easing(&easing::STANDARD)
                    .cycle_duration(Duration::from_secs_f32(2.0))
                    .width(Length::Fill),
            )
            .into();
        }

        let list = SelectionList::new_with(
            &self.resources,
            Message::ResourceSelected,
            14.0,
            5.0,
            iced_aw::style::selection_list::primary,
            None,
            get_default_font(),
        )
        .style(styles::resource_selection_list)
        .width(Length::Fill)
        .height(Length::Fill);

        container(list)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}
