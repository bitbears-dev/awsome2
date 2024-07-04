use crate::{error::Error, pane_type::PaneType, workspace::Workspace};

#[derive(Clone, Debug)]
pub struct State {
    side_drawer_open: bool,
    side_drawer_width: f32,
    active_pane: PaneType,
}

impl State {
    pub fn new() -> Self {
        Self {
            side_drawer_open: false,
            side_drawer_width: 150.0,
            active_pane: PaneType::Explore,
        }
    }

    pub fn from_workspace(_ws: Workspace) -> Result<Self, Error> {
        Ok(Self {
            side_drawer_open: false,
            side_drawer_width: 150.0,
            active_pane: PaneType::Explore,
        })
    }

    pub fn is_side_drawer_open(&self) -> bool {
        self.side_drawer_open
    }

    pub fn close_side_drawer(&mut self) {
        self.side_drawer_open = false;
    }

    pub fn toggle_side_drawer(&mut self) {
        self.side_drawer_open = !self.side_drawer_open;
    }

    pub fn get_side_drawer_width(&self) -> f32 {
        match self.side_drawer_open {
            true => self.side_drawer_width,
            false => 40.0,
        }
    }

    pub fn get_active_pane(&self) -> PaneType {
        self.active_pane
    }

    pub fn set_active_pane(&mut self, pane_type: PaneType) {
        self.active_pane = pane_type;
    }
}
