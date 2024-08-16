use aws_config::{BehaviorVersion, Region};
use lazy_static::lazy_static;

use crate::{error::Error, service::Service};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Resource {
    LambdaFunction(Box<LambdaFunctionInfo>),
    S3Bucket(BucketInfo),
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

pub async fn load_resources(
    profile: String,
    region: String,
    service: &Service,
) -> Result<Vec<Resource>, Error> {
    match service {
        Service::Lambda => load_lambda_functions(profile, region).await,
        Service::S3 => load_s3_buckets(profile, region).await,
    }
}

async fn load_lambda_functions(profile: String, region: String) -> Result<Vec<Resource>, Error> {
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
        .map(|f| Resource::LambdaFunction(Box::new(LambdaFunctionInfo(f))))
        .collect())
}

async fn load_s3_buckets(profile: String, region: String) -> Result<Vec<Resource>, Error> {
    let cfg = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .profile_name(profile)
        .region(Region::new(region.to_string()))
        .load()
        .await;

    let client = aws_sdk_s3::Client::new(&cfg);
    let out = client.list_buckets().send().await?;

    let mut buckets = out.buckets().to_owned();
    buckets.sort_by_key(|b| b.name.clone());

    Ok(buckets
        .into_iter()
        .map(|b| Resource::S3Bucket(BucketInfo(b)))
        .collect())
}
