use serde::{Deserialize, Serialize};

use crate::workspace::resource::Resource;

#[derive(Debug, Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Project {
    pub name: String,
    pub resources: Vec<Resource>,
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
