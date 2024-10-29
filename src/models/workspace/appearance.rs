use serde::{Deserialize, Serialize};

use super::{resource_descriptor::ResourceDescriptor, Project};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Appearance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_project: Option<Project>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_service: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_resource: Option<ResourceDescriptor>,
}
