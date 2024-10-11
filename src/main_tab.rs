use iced::{
    widget::{button, column, container, row, text, Space},
    Element, Theme,
};

use crate::{
    bootstrap_text::bootstrap_text, explore_tab::ExploreTab, footer::Footer, message::Message,
    pane_type::PaneType, projects_tab::ProjectsTab, state::State, styles,
};

pub struct MainTab {
    pub explore_tab: ExploreTab,
    pub projects_tab: ProjectsTab,
    pub footer: Footer,
}

impl MainTab {
    pub fn new() -> Self {
        Self {
            explore_tab: ExploreTab::new(),
            projects_tab: ProjectsTab::new(),
            footer: Footer::new(),
        }
    }

    pub fn view<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        let r = row![
            self.render_side_drawer(state),
            self.render_main_tab_pane(state)
        ];

        let c = column![self.render_header(state), r, self.footer.view(state)];

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
        .align_y(iced::alignment::Vertical::Bottom)
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
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style::default().background(palette.background.weak.color)
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
                    .align_y(iced::alignment::Vertical::Center)
            ],
            false => row![icon],
        };
        let button_style = match state.get_active_pane() {
            PaneType::Explore => styles::side_drawer_button_selected,
            _ => styles::side_drawer_button,
        };
        button(button_content)
            .on_press(Message::ActivateExploreTab)
            .width(iced::Length::Fill)
            .style(button_style)
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
                    .align_y(iced::alignment::Vertical::Center)
            ],
            false => row![icon],
        };
        let button_style = match state.get_active_pane() {
            PaneType::Projects => styles::side_drawer_button_selected,
            _ => styles::side_drawer_button,
        };
        button(button_content)
            .on_press(Message::ActivateProjectsTab)
            .width(iced::Length::Fill)
            .style(button_style)
            .into()
    }

    fn render_main_tab_pane<'a>(&'a self, state: &'a State) -> Element<'a, Message> {
        match state.get_active_pane() {
            PaneType::Explore => self.explore_tab.view(),
            PaneType::Projects => self.projects_tab.view(),
        }
    }
}
