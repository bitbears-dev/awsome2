use crate::{error::Error, models::workspace::Workspace, view::pane_type::PaneType};

#[derive(Clone, Debug)]
pub struct State {
    pub workspace: Workspace,
    nearest_region: String,
    side_drawer_open: bool,
    side_drawer_width: f32,
    active_pane: PaneType,
    //log_sender: Option<iced::futures::channel::mpsc::Sender<String>>,
    logs: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            workspace: Workspace::default(),
            nearest_region: String::from("ap-northeast-1"), // TODO: get from config
            side_drawer_open: false,
            side_drawer_width: 150.0,
            active_pane: PaneType::Explore,
            //log_sender: None,
            logs: Vec::new(),
        }
    }

    pub fn from_workspace(ws: Workspace) -> Result<Self, Error> {
        Ok(Self {
            workspace: ws,
            nearest_region: String::from("ap-northeast-1"), // TODO: get from config
            side_drawer_open: false,
            side_drawer_width: 150.0,
            active_pane: PaneType::Explore,
            //log_sender: None,
            logs: Vec::new(),
        })
    }

    pub fn get_nearest_region(&self) -> &str {
        &self.nearest_region
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

    //pub fn set_log_sender(&mut self, log_sender: Sender<String>) {
    //    self.log_sender = Some(log_sender);
    //}

    pub fn append_log(&mut self, log: String) {
        self.logs.push(log);
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.clone()
    }
}
