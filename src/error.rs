use std::convert::From;

#[derive(Clone, Debug)]
pub enum Error {
    UnknownError,
    UnableToLoadAwsConfig,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Self {
        Error::UnknownError
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(_e: serde_yaml::Error) -> Self {
        Error::UnknownError
    }
}

impl<E: std::fmt::Debug, R: std::fmt::Debug>
    std::convert::From<aws_smithy_runtime_api::client::result::SdkError<E, R>> for Error
{
    fn from(e: aws_smithy_runtime_api::client::result::SdkError<E, R>) -> Error {
        println!("{:?}", e);
        Error::UnknownError
    }
}
