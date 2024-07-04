use iced::{
    widget::{button, column, container, row, text, Space},
    Color, Element,
};

use crate::{bootstrap_text::bootstrap_text, message::Message, pane_type::PaneType, state::State};

pub struct MainTab {}

impl MainTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, state: &State) -> Element<Message> {
        let mut c = column![self.render_header(state)];

        let mut r = row![self.render_side_drawer(state)];

        r = r.push(self.render_main_tab_pane(state));

        c = c.push(r);

        let cont: Element<Message> = container(c).into();
        //let cont = cont.explain(Color::from_rgb(255.0, 0.0, 0.0));
        cont
    }

    fn render_header(&self, state: &State) -> Element<Message> {
        row![
            self.render_menu_button(),
            self.render_spacer(),
            self.render_current_active_pane_name(state)
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Shrink)
        .into()
    }

    fn render_menu_button(&self) -> Element<Message> {
        let icon = bootstrap_text(
            "\u{F479}",
            24.0,
            iced::Length::Fixed(32.0),
            iced::Length::Fixed(32.0),
        );
        button(icon)
            .width(iced::Length::Shrink)
            .height(iced::Length::Shrink)
            .on_press(Message::SideDrawerToggled)
            .into()
    }

    fn render_spacer(&self) -> Element<Message> {
        container(Space::new(8.0, 8.0)).into()
    }

    fn render_current_active_pane_name(&self, state: &State) -> Element<Message> {
        text(match state.get_active_pane() {
            PaneType::Explore => "Explore",
            PaneType::Projects => "Projects",
        })
        .vertical_alignment(iced::alignment::Vertical::Bottom)
        .height(iced::Length::Fixed(32.0))
        .into()
    }

    fn render_side_drawer(&self, state: &State) -> Element<Message> {
        container(column![
            self.render_explore_button(state),
            self.render_projects_button(state)
        ])
        .width(state.get_side_drawer_width())
        .height(iced::Length::Fill)
        .style(container::Appearance {
            background: Some(iced::Background::Color(Color::from_rgb8(0x33, 0x33, 0x33))),
            ..Default::default()
        })
        .into()
    }

    fn render_explore_button(&self, state: &State) -> Element<Message> {
        let icon = bootstrap_text(
            "\u{F2D1}",
            24.0,
            iced::Length::Fixed(32.0),
            iced::Length::Fixed(32.0),
        );
        let button_content = match state.is_side_drawer_open() {
            true => row![
                icon,
                text("Explore")
                    .height(iced::Length::Fixed(32.0))
                    .vertical_alignment(iced::alignment::Vertical::Center)
            ],
            false => row![icon],
        };
        button(button_content)
            .on_press(Message::ActivateExploreTab)
            .width(iced::Length::Fill)
            .into()
    }

    fn render_projects_button(&self, state: &State) -> Element<Message> {
        let icon = bootstrap_text(
            "\u{F477}",
            24.0,
            iced::Length::Fixed(32.0),
            iced::Length::Fixed(32.0),
        );
        let button_content = match state.is_side_drawer_open() {
            true => row![
                icon,
                text("Projects")
                    .height(iced::Length::Fixed(32.0))
                    .vertical_alignment(iced::alignment::Vertical::Center)
            ],
            false => row![icon],
        };
        button(button_content)
            .on_press(Message::ActivateProjectsTab)
            .width(iced::Length::Fill)
            .into()
    }

    fn render_main_tab_pane(&self, state: &State) -> Element<Message> {
        match state.get_active_pane() {
            PaneType::Explore => self.render_explore_tab(),
            PaneType::Projects => self.render_projects_tab(),
        }
    }

    fn render_explore_tab(&self) -> Element<Message> {
        text("Explore Tab")
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    fn render_projects_tab(&self) -> Element<Message> {
        text("Projects Tab").into()
    }
}
