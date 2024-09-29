use serde::{Deserialize, Serialize};

use crate::service::Service;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Resource {
    pub profile: String,
    pub region: String,
    pub service: Service,
    pub id: String,
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_display_name())
    }
}

impl Resource {
    pub fn get_display_name(&self) -> String {
        match self.service {
            Service::Lambda => self.id.to_string(),
            Service::S3 => self.id.to_string(),
        }
    }
}
