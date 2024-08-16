use std::time::Duration;

use iced::{
    widget::{column, container, pick_list, row, text},
    Element, Length,
};
use iced_aw::{SelectionList, SelectionListStyles};

use crate::{
    easing,
    fonts::get_default_font,
    lambda_function_details::LambdaFunctionDetails,
    linear::Linear,
    message::Message,
    profiles::load_profiles,
    resource::{BucketInfo, Resource},
    service::Service,
    splitter::Splitter,
};

const SERVICES: &[&Service] = &[&Service::Lambda, &Service::S3];

pub struct ExploreTab {
    profiles: Vec<String>,
    selected_profile: Option<String>,
    regions: Vec<String>,
    selected_region: Option<String>,
    services: Vec<&'static Service>,
    selected_service: Option<&'static Service>,
    service_selector_width: f32,
    pub splitter1: Splitter,
    loading_resources: bool,
    resources: Vec<Resource>,
    selected_resource: Option<Resource>,
    resource_selector_width: f32,
    pub splitter2: Splitter,
    lambda_function_details: LambdaFunctionDetails,
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
            service_selector_width: 150.0,
            splitter1: Splitter::new(150.0, 100.0, 300.0, Message::Splitter1Moved),
            loading_resources: false,
            resources: vec![],
            selected_resource: None,
            resource_selector_width: 150.0,
            splitter2: Splitter::new(150.0, 100.0, 300.0, Message::Splitter2Moved),
            lambda_function_details: LambdaFunctionDetails::new(),
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

    pub fn set_service_selector_width(&mut self, width: f32) {
        self.service_selector_width = width;
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
        self.selected_resource = Some(resource);
    }

    pub fn set_resource_selector_width(&mut self, width: f32) {
        self.resource_selector_width = width;
    }

    pub fn view(&self) -> Element<Message> {
        let service_selector = self.render_service_selector();
        let splitter1 = self.splitter1.view().map(Message::Splitter1Event);
        let mut r = row![service_selector, splitter1];
        if self.selected_service.is_some() {
            let resource_selector = self.render_resource_selector();
            r = r.push(resource_selector);

            let splitter2 = self.splitter2.view().map(Message::Splitter2Event);
            r = r.push(splitter2);

            if self.selected_resource.is_some() {
                let resource_details = self.render_resource_details();
                r = r.push(resource_details);
            }
        }
        container(r)
            .padding(4.0)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn render_service_selector(&self) -> Element<Message> {
        let profile_selector = pick_list(
            self.profiles.clone(),
            self.selected_profile.clone(),
            Message::ProfileSelected,
        )
        .width(Length::Fill);
        let mut c = column![profile_selector]
            .width(iced::Length::Fixed(self.service_selector_width))
            .height(Length::Fill);

        if self.selected_profile.is_some() {
            let region_selector = pick_list(
                self.regions.clone(),
                self.selected_region.clone(),
                Message::RegionSelected,
            )
            .width(Length::Fill);
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
            .width(Length::Fill)
            .height(Length::Fill);
            c = c.push(service_selector);
        }
        container(c)
            .width(iced::Length::Shrink)
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
            SelectionListStyles::Default,
            None,
            get_default_font(),
        )
        .width(Length::Fixed(self.resource_selector_width))
        .height(Length::Fill);

        container(list)
            .width(iced::Length::Shrink)
            .height(iced::Length::Fill)
            .into()
    }

    fn render_resource_details(&self) -> Element<Message> {
        match &self.selected_resource {
            Some(resource) => match resource {
                Resource::LambdaFunction(f) => {
                    return self.lambda_function_details.render(f);
                }
                Resource::S3Bucket(b) => {
                    return self.render_s3_bucket_details(b);
                }
            },
            None => container(text("No resource selected").size(24)).into(),
        }
    }

    fn render_s3_bucket_details(&self, b: &BucketInfo) -> Element<Message> {
        let mut c = column![];
        c = c.push(text("S3 Bucket Details").size(24));
        c = c.push(text(format!(
            "Name: {}",
            b.0.name.as_ref().unwrap_or(&"Unnamed".to_string())
        )));
        c.into()
    }
}
