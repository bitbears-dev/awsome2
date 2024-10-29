use iced::{
    widget::{column, container, scrollable, text, Column},
    Element,
};

use crate::{message::Message, state::State};

pub struct Footer {}

impl Footer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self, state: &State) -> Element<Message> {
        let mut logs: Column<'_, Message> = column![].align_x(iced::alignment::Alignment::Start);

        for log in state.get_logs().iter() {
            logs = logs.push(text(log.to_string()));
        }

        let s = scrollable(logs)
            .direction(scrollable::Direction::Vertical(
                scrollable::Scrollbar::new()
                    .width(15)
                    .margin(5)
                    .scroller_width(10)
                    .anchor(scrollable::Anchor::Start),
            ))
            .width(iced::Length::Fill)
            .height(iced::Length::Fixed(100.0));

        container(s)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .into()
    }
}
