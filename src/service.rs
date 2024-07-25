#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Service {
    Lambda,
    S3,
}

impl std::fmt::Display for Service {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Lambda => "Lambda",
            Self::S3 => "S3",
        };
        write!(f, "{}", name)
    }
}