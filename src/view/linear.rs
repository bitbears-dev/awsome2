// inspired by https://github.com/iced-rs/iced/blob/7a50e9e8fbb8d37e53a42c1dd5936b97463ead53/examples/loading_spinners/src/linear.rs
use std::time::Duration;

use iced::{
    advanced::{
        layout,
        renderer::{self, Quad},
        widget::{tree, Tree},
        Clipboard, Layout, Shell, Widget,
    },
    event, mouse,
    time::Instant,
    window::{self, RedrawRequest},
    Background, Color, Element, Event, Length, Rectangle, Size,
};

use crate::view::easing::{self, Easing};

pub struct Linear<'a, Theme>
where
    Theme: StyleSheet,
{
    width: Length,
    height: Length,
    style: Theme::Style,
    easing: &'a Easing,
    cycle_duration: Duration,
}

impl<'a, Theme> Linear<'a, Theme>
where
    Theme: StyleSheet,
{
    pub fn new() -> Self {
        Linear {
            width: Length::Fixed(100.0),
            height: Length::Fixed(4.0),
            style: Theme::Style::default(),
            easing: &easing::STANDARD,
            cycle_duration: Duration::from_millis(600),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn style(mut self, style: impl Into<Theme::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn easing(mut self, easing: &'a Easing) -> Self {
        self.easing = easing;
        self
    }

    pub fn cycle_duration(mut self, duration: Duration) -> Self {
        self.cycle_duration = duration / 2;
        self
    }
}

impl<Theme> Default for Linear<'_, Theme>
where
    Theme: StyleSheet,
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
enum State {
    Expanding { start: Instant, progress: f32 },
    Contracting { start: Instant, progress: f32 },
}

impl Default for State {
    fn default() -> Self {
        Self::Expanding {
            start: Instant::now(),
            progress: 0.0,
        }
    }
}

impl State {
    fn next(&self, now: Instant) -> Self {
        match self {
            Self::Expanding { .. } => Self::Contracting {
                start: now,
                progress: 0.0,
            },
            Self::Contracting { .. } => Self::Expanding {
                start: now,
                progress: 0.0,
            },
        }
    }

    fn start(&self) -> Instant {
        match self {
            Self::Expanding { start, .. } | Self::Contracting { start, .. } => *start,
        }
    }

    fn timed_transition(&self, cycle_duration: Duration, now: Instant) -> Self {
        let elapsed = now.duration_since(self.start());

        match elapsed {
            elapsed if elapsed > cycle_duration => self.next(now),
            _ => self.with_elapsed(cycle_duration, elapsed),
        }
    }

    fn with_elapsed(&self, cycle_duration: Duration, elapsed: Duration) -> Self {
        let progress = elapsed.as_secs_f32() / cycle_duration.as_secs_f32();
        match self {
            Self::Expanding { start, .. } => Self::Expanding {
                start: *start,
                progress,
            },
            Self::Contracting { start, .. } => Self::Contracting {
                start: *start,
                progress,
            },
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Linear<'a, Theme>
where
    Message: Clone + 'a,
    Theme: StyleSheet + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> event::Status {
        let state = tree.state.downcast_mut::<State>();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            *state = state.timed_transition(self.cycle_duration, now);

            shell.request_redraw(RedrawRequest::NextFrame);
        }

        event::Status::Ignored
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let custom_style = theme.appearance(&self.style);
        let state = tree.state.downcast_ref::<State>();

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: bounds.y,
                    width: bounds.width,
                    height: bounds.height,
                },
                ..renderer::Quad::default()
            },
            Background::Color(custom_style.track_color),
        );

        match state {
            State::Expanding { progress, .. } => renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: bounds.x,
                        y: bounds.y,
                        width: self.easing.y_at_x(*progress) * bounds.width,
                        height: bounds.height,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(custom_style.bar_color),
            ),

            State::Contracting { progress, .. } => renderer.fill_quad(
                Quad {
                    bounds: Rectangle {
                        x: bounds.x + self.easing.y_at_x(*progress) * bounds.width,
                        y: bounds.y,
                        width: (1.0 - self.easing.y_at_x(*progress)) * bounds.width,
                        height: bounds.height,
                    },
                    ..renderer::Quad::default()
                },
                Background::Color(custom_style.bar_color),
            ),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Linear<'a, Theme>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Theme: StyleSheet + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(linear: Linear<'a, Theme>) -> Self {
        Self::new(linear)
    }
}
pub struct Appearance {
    pub track_color: Color,
    pub bar_color: Color,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            track_color: Color::TRANSPARENT,
            bar_color: Color::BLACK,
        }
    }
}
pub trait StyleSheet {
    type Style: Default;
    fn appearance(&self, style: &Self::Style) -> Appearance;
}

impl StyleSheet for iced::Theme {
    type Style = ();

    fn appearance(&self, _style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

        Appearance {
            track_color: palette.background.weak.color,
            bar_color: palette.primary.base.color,
        }
    }
}
