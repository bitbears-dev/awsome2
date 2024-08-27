use iced::{
    border::Radius,
    theme::palette::Extended,
    widget::{container, mouse_area, text},
    Element, Theme,
};

use crate::message::Message;

struct SplitterState {
    pos: f32,
    is_dragging: bool,
    hover: bool,
    last_pos: Option<iced::Point>,
}

#[derive(Clone, Debug)]
pub enum Event {
    Enter,
    Exit,
    Press,
    Release,
    Move(iced::Point),
}

pub struct Splitter {
    min: f32,
    max: f32,
    message: Box<dyn Fn(f32) -> Message>,
    state: SplitterState,
}

impl Splitter {
    pub fn new(pos: f32, min: f32, max: f32, message: fn(f32) -> Message) -> Self {
        Self {
            min,
            max,
            message: Box::new(message),
            state: SplitterState {
                pos,
                is_dragging: false,
                hover: false,
                last_pos: None,
            },
        }
    }

    pub fn update(&mut self, event: Event) -> Option<Message> {
        //let event = dbg!(event);
        match event {
            Event::Enter => {
                self.state.hover = true;
            }
            Event::Exit => {
                self.state.hover = false;
                self.state.is_dragging = false;
                self.state.last_pos = None;
            }
            Event::Press => {
                self.state.is_dragging = true;
                self.state.last_pos = None;
            }
            Event::Release => {
                self.state.is_dragging = false;
                self.state.last_pos = None;
            }
            Event::Move(pos) => match self.state.last_pos {
                Some(last_pos) => {
                    let delta = pos.x - last_pos.x;
                    self.state.pos += delta;
                    self.state.pos = self.state.pos.clamp(self.min, self.max);
                    //self.state.pos = dbg!(self.state.pos);
                    return Some((self.message)(self.state.pos));
                }
                None => {
                    if self.state.is_dragging {
                        self.state.last_pos = Some(pos);
                    }
                }
            },
        }
        None
    }

    pub fn view(&self) -> Element<Event> {
        container(
            mouse_area(
                text(" ")
                    .width(iced::Length::Fill)
                    .height(iced::Length::Fill),
            )
            .on_enter(Event::Enter)
            .on_exit(Event::Exit)
            .on_press(Event::Press)
            .on_release(Event::Release)
            .on_move(Event::Move)
            .interaction(iced::mouse::Interaction::ResizingHorizontally),
        )
        .width(iced::Length::Fixed(12.0))
        .height(iced::Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style::default()
                .background(self.get_background_color(&palette))
                .border(iced::Border {
                    color: palette.primary.base.color,
                    width: 1.0,
                    radius: Radius::default(),
                })
        })
        .into()
    }

    fn get_background_color(&self, palette: &Extended) -> iced::Background {
        if self.state.hover {
            iced::Background::Color(iced::Color::from_rgb8(0x00, 0x00, 0xf0))
        } else {
            palette.background.base.color.into()
        }
    }
}
