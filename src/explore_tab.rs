use iced::{widget::{column, container, pick_list, row}, Element, Length};
use iced_aw::{SelectionList, SelectionListStyles};

use crate::{fonts::get_default_font, message::Message, profiles::load_profiles, resource::Resource, service::Service};

const SERVICES: &[&Service] = &[&Service::Lambda, &Service::S3];

pub struct ExploreTab {
    profiles: Vec<String>,
    selected_profile: Option<String>,
    regions: Vec<String>,
    selected_region: Option<String>,
    services: Vec<&'static Service>,
    selected_service: Option<&'static Service>,
    resources: Vec<Resource>,
    selected_resource: Option<Resource>,
}

impl ExploreTab {
    pub fn new() -> Self {
        let profiles = load_profiles().unwrap_or_else(|err| {
            eprintln!("Failed to load profiles: {:?}", err);
            vec![]
        });
        Self {
            profiles,
            selected_profile: None,
            regions: vec![],
            selected_region: None,
            services: SERVICES.to_vec(),
            selected_service: None,
            resources: vec![],
            selected_resource: None,
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

    pub fn set_resources(&mut self, resources: Vec<Resource>) {
        self.resources = resources;
    }

    pub fn set_selected_resource(&mut self, resource: Resource) {
        self.selected_resource = Some(resource);
    }

    pub fn view(&self) -> Element<Message> {
        let service_selector = self.render_service_selector();
        let mut r = row![service_selector];
        if self.selected_service.is_some() {
            let resource_selector = self.render_resource_selector();
            r = r.push(resource_selector);
        }
        container(r).padding(4.0).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn render_service_selector(&self) -> Element<Message> {
        let profile_selector = pick_list(self.profiles.clone(), self.selected_profile.clone(), Message::ProfileSelected);
        let mut c = column![profile_selector].width(iced::Length::Shrink).height(iced::Length::Fill);

        if self.selected_profile.is_some() {
            let region_selector = pick_list(self.regions.clone(), self.selected_region.clone(), Message::RegionSelected);
            c = c.push(region_selector);
        }

        if self.selected_region.is_some() {
            let service_selector = SelectionList::new_with(
                &self.services,
                Message::ServiceSelected,
                14.0,
                5.0,
                SelectionListStyles::Default,
                None,
                get_default_font(),
            )
            .width(Length::Shrink)
            .height(Length::Fill);
            c = c.push(service_selector);
        }
        container(c).width(iced::Length::Shrink).height(iced::Length::Fill).into()
    }

    fn render_resource_selector(&self) -> Element<Message> {
        let list = SelectionList::new_with(
            &self.resources,
            Message::ResourceSelected,
            14.0,
            5.0,
            SelectionListStyles::Default,
            None,
            get_default_font(),
        )
        .width(Length::Shrink)
        .height(Length::Fill);

        container(list).width(iced::Length::Shrink).height(iced::Length::Fill).into()
    }
}
