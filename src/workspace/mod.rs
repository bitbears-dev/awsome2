pub mod project;
pub mod resource;

pub use project::Project;
pub use resource::Resource;

use std::{fs::File, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::Error;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Workspace {
    pub projects: Vec<Project>,
}

impl Workspace {
    pub async fn load(workspace_file: Option<PathBuf>) -> Result<Self, Error> {
        match workspace_file {
            Some(workspace_file) => Ok(Self::load_workspace_file(workspace_file).await?),
            None => Ok(Self::load_empty_workspace().await?),
        }
    }

    async fn load_workspace_file(workspace_file: PathBuf) -> Result<Self, Error> {
        let f = File::open(workspace_file)?;
        let workspace: Workspace = serde_yaml::from_reader(f)?;
        Ok(workspace)
    }

    async fn load_empty_workspace() -> Result<Self, Error> {
        let workspace = Workspace::default();
        Ok(workspace)
    }
}
