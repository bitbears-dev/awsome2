use aws_config::{BehaviorVersion, Region};
use lazy_static::lazy_static;

use crate::{
    error::Error,
    models::{service::Service, workspace::ResourceDescriptor},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    LambdaFunction(Box<LambdaFunctionInfo>),
    S3Bucket(BucketInfo),
}

impl Resource {
    pub async fn load(resource_descriptor: ResourceDescriptor) -> Result<Self, Error> {
        match resource_descriptor.service {
            Service::Lambda => Self::load_lambda_function(&resource_descriptor).await,
            Service::S3 => Self::load_s3_bucket(&resource_descriptor).await,
        }
    }

    async fn load_lambda_function(rd: &ResourceDescriptor) -> Result<Resource, Error> {
        match rd.service {
            Service::Lambda => {
                let cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
                    .profile_name(rd.profile.clone())
                    .region(Region::new(rd.region.to_string()))
                    .load()
                    .await;

                let client = aws_sdk_lambda::Client::new(&cfg);
                let out = client
                    .get_function()
                    .function_name(rd.id.clone())
                    .send()
                    .await?;

                let Some(function_config) = out.configuration else {
                    return Err(Error::ResourceNotFound);
                };
                Ok(Resource::LambdaFunction(Box::new(LambdaFunctionInfo(
                    function_config,
                ))))
            }
            _ => Err(Error::InvalidResourceDescriptor),
        }
    }

    pub async fn list_lambda_functions(
        profile: String,
        region: String,
    ) -> Result<Vec<Self>, Error> {
        let cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .profile_name(profile)
            .region(Region::new(region.to_string()))
            .load()
            .await;

        let client = aws_sdk_lambda::Client::new(&cfg);
        let result: Result<Vec<_>, _> = client
            .list_functions()
            .into_paginator()
            .items()
            .send()
            .collect()
            .await;

        let mut functions = result?;
        functions.sort_by_key(|f| f.function_name.clone());
        Ok(functions
            .into_iter()
            .map(|f| Self::LambdaFunction(Box::new(LambdaFunctionInfo(f))))
            .collect())
    }

    pub async fn load_s3_bucket(rd: &ResourceDescriptor) -> Result<Self, Error> {
        let cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .profile_name(rd.profile.clone())
            .region(Region::new(rd.region.clone()))
            .load()
            .await;

        let client = aws_sdk_s3::Client::new(&cfg);
        let out = client.list_buckets().send().await?;

        let bucket = out
            .buckets()
            .iter()
            .find(|b| b.name() == Some(&rd.id))
            .ok_or(Error::ResourceNotFound)?;

        Ok(Self::S3Bucket(BucketInfo(bucket.to_owned())))
    }

    pub async fn list_s3_buckets(profile: String, region: String) -> Result<Vec<Self>, Error> {
        let cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
            .profile_name(profile)
            .region(Region::new(region))
            .load()
            .await;

        let client = aws_sdk_s3::Client::new(&cfg);
        let out = client.list_buckets().send().await?;

        let mut buckets = out.buckets().to_owned();
        buckets.sort_by_key(|b| b.name.clone());

        Ok(buckets
            .into_iter()
            .map(|b| Self::S3Bucket(BucketInfo(b)))
            .collect())
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

pub async fn list_resources(
    profile: String,
    region: String,
    service: &Service,
) -> Result<Vec<Resource>, Error> {
    match service {
        Service::Lambda => Resource::list_lambda_functions(profile, region).await,
        Service::S3 => Resource::list_s3_buckets(profile, region).await,
    }
}
