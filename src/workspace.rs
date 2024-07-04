use std::{fs::File, path::PathBuf};

use serde::Deserialize;

use crate::{error::Error, state::State};

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Workspace {}

pub async fn load(workspace_file: Option<PathBuf>) -> Result<State, Error> {
    match workspace_file {
        Some(workspace_file) => Ok(load_workspace_file(workspace_file).await?),
        None => Ok(load_empty_workspace().await?),
    }
}

async fn load_workspace_file(workspace_file: PathBuf) -> Result<State, Error> {
    let f = File::open(workspace_file)?;
    let workspace: Workspace = serde_yaml::from_reader(f)?;
    let state = State::from_workspace(workspace)?;
    Ok(state)
}

async fn load_empty_workspace() -> Result<State, Error> {
    let workspace = Workspace::default();
    let state = State::from_workspace(workspace)?;
    Ok(state)
}
