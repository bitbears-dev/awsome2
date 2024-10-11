use std::convert::From;

use aws_smithy_runtime_api::client::result::SdkError;

#[derive(Clone, Debug)]
pub enum Error {
    Unknown,
    UnableToLoadAwsConfig,
    AwsSdk(String),
    InvalidResourceDescriptor,
    ResourceNotFound,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Unknown => write!(f, "Unknown error"),
            Error::UnableToLoadAwsConfig => write!(f, "Unable to load AWS config"),
            Error::AwsSdk(msg) => write!(f, "AWS SDK error: {}", msg),
            Error::InvalidResourceDescriptor => write!(f, "Invalid resource descriptor"),
            Error::ResourceNotFound => write!(f, "Resource not found"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(_e: std::io::Error) -> Self {
        Error::Unknown
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(_e: serde_yaml::Error) -> Self {
        Error::Unknown
    }
}

impl<E: std::fmt::Debug, R: std::fmt::Debug>
    std::convert::From<aws_smithy_runtime_api::client::result::SdkError<E, R>> for Error
{
    fn from(e: SdkError<E, R>) -> Error {
        let err_message = format!("{:?}", e);
        eprintln!("{}", err_message);
        match e {
            SdkError::ConstructionFailure(_) => Error::AwsSdk(err_message),
            SdkError::TimeoutError(_) => Error::AwsSdk(err_message),
            SdkError::DispatchFailure(_) => Error::AwsSdk(err_message),
            SdkError::ResponseError(_) => Error::AwsSdk(err_message),

            SdkError::ServiceError(_service_err) => {
                //let err = service_err.into_err();
                Error::Unknown
            }

            _ => Error::Unknown,
        }
    }
}
