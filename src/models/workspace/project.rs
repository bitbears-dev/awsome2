use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use crate::{models::workspace::resource_descriptor::ResourceDescriptor, service::Service};

#[derive(Debug, Clone, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Project {
    id: String,
    pub name: String,
    pub resources: Vec<ResourceDescriptor>,
}

impl std::fmt::Display for Project {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Self {
            id: ulid::Ulid::new().to_string(),
            name: name.to_string(),
            resources: vec![],
        }
    }

    pub fn get_services(&self) -> Vec<Service> {
        self.resources
            .iter()
            .map(|resource| resource.service.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>()
    }
}
