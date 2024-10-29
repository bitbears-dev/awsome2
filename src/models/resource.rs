use lazy_static::lazy_static;

use crate::{
    error::Error,
    models::{service::Service, workspace::ResourceDescriptor},
    view::resource_details::{
        lambda_function_details::LambdaFunctionDetails, s3_bucket_details::S3BucketDetails,
    },
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    LambdaFunction(Box<LambdaFunctionInfo>),
    S3Bucket(BucketInfo),
}

impl Resource {
    pub async fn load(resource_descriptor: ResourceDescriptor) -> Result<Self, Error> {
        match resource_descriptor.service {
            Service::Lambda => LambdaFunctionDetails::load(&resource_descriptor).await,
            Service::S3 => S3BucketDetails::load(&resource_descriptor).await,
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub struct LambdaFunctionInfo(pub aws_sdk_lambda::types::FunctionConfiguration);

impl Eq for LambdaFunctionInfo {}

impl std::hash::Hash for LambdaFunctionInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.function_name.hash(state);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BucketInfo(pub aws_sdk_s3::types::Bucket);

impl Eq for BucketInfo {}

impl std::hash::Hash for BucketInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.name.hash(state);
    }
}

lazy_static! {
    static ref UNNAMED: String = "Unnamed".to_string();
}

impl std::fmt::Display for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LambdaFunction(info) => {
                write!(f, "{}", info.0.function_name.as_ref().unwrap_or(&UNNAMED))
            }
            Self::S3Bucket(info) => write!(f, "{}", info.0.name.as_ref().unwrap_or(&UNNAMED)),
        }
    }
}

pub async fn load_resource(resource_descriptor: ResourceDescriptor) -> Result<Resource, Error> {
    Resource::load(resource_descriptor).await
}

pub async fn list_resources(
    profile: String,
    region: String,
    service: &Service,
) -> Result<Vec<Resource>, Error> {
    match service {
        Service::Lambda => LambdaFunctionDetails::list(profile, region).await,
        Service::S3 => S3BucketDetails::list(profile, region).await,
    }
}
