mod appearance;
pub mod project;
pub mod resource_descriptor;

use crate::workspace::appearance::Appearance;
pub use project::Project;
pub use resource_descriptor::ResourceDescriptor;

use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workspace {
    path: Option<PathBuf>,
    pub projects: Vec<Project>,
    pub appearance: Appearance,
}

impl Workspace {
    pub async fn load(workspace_file: Option<PathBuf>) -> Result<Self, Error> {
        match workspace_file {
            Some(workspace_file) => Ok(Self::load_workspace_file(workspace_file).await?),
            None => Ok(Self::load_empty_workspace().await?),
        }
    }

    async fn load_workspace_file(workspace_file: PathBuf) -> Result<Self, Error> {
        let f = File::open(workspace_file.clone())?;
        let mut workspace: Workspace = serde_yaml::from_reader(f)?;
        workspace.path = Some(workspace_file);
        Ok(workspace)
    }

    async fn load_empty_workspace() -> Result<Self, Error> {
        let workspace = Workspace::default();
        Ok(workspace)
    }

    pub fn set_selected_resource(&mut self, resource: Option<ResourceDescriptor>) {
        self.appearance.selected_resource = resource;
        self.save();
    }

    pub fn save(&self) {
        let f = File::create("workspace.yaml").unwrap();
        serde_yaml::to_writer(f, self).unwrap();
    }
}
