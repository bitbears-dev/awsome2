mod appearance;
pub mod project;
pub mod resource_descriptor;

pub use project::Project;
pub use resource_descriptor::ResourceDescriptor;

use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{error::Error, models::workspace::appearance::Appearance};

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

    pub fn set_selected_resource(
        &mut self,
        resource: Option<ResourceDescriptor>,
    ) -> Result<(), Error> {
        self.appearance.selected_resource = resource;
        self.save()?;
        Ok(())
    }

    pub fn add_project(&mut self, project: Project) -> Result<(), Error> {
        self.projects.push(project);
        self.save()?;
        Ok(())
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = self.path.clone().unwrap_or(PathBuf::from("workspace.yaml"));
        let f = File::create(path)?;
        serde_yaml::to_writer(f, self)?;
        Ok(())
    }
}
