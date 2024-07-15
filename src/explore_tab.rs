use iced::{widget::{column, container, pick_list, row, text}, Element};

use crate::{message::Message, profiles::load_profiles};

pub struct ExploreTab {
    profiles: Vec<String>,
    selected_profile: Option<String>,
    regions: Vec<String>,
    selected_region: Option<String>,
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
        }
    }

    pub fn set_selected_profile(&mut self, profile: String) {
        self.selected_profile = Some(profile);
    }

    pub fn set_regions(&mut self, regions: Vec<String>) {
        self.regions = regions;
    }

    pub fn set_selected_region(&mut self, region: String) {
        self.selected_region = Some(region);
    }

    pub fn view(&self) -> Element<Message> {
        let service_selector = self.render_service_selector();
        let r = row![service_selector];
        container(r).padding(4.0).width(iced::Length::Fill).height(iced::Length::Fill).into()
    }

    fn render_service_selector(&self) -> Element<Message> {
        let profile_selector = pick_list(self.profiles.clone(), self.selected_profile.clone(), Message::ProfileSelected);
        let mut c = column![profile_selector].width(iced::Length::Shrink).height(iced::Length::Fill);

        if self.selected_profile.is_some() {
            let region_selector = pick_list(self.regions.clone(), self.selected_region.clone(), Message::RegionSelected);
            c = c.push(region_selector);
        }

        container(c).width(iced::Length::Shrink).height(iced::Length::Fill).into()
    }

}
