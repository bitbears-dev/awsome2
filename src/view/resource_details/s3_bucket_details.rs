use aws_config::{BehaviorVersion, Region};
use iced::{
    widget::{column, text},
    Element,
};

use crate::{
    error::Error,
    message::Message,
    models::{
        resource::{BucketInfo, Resource},
        workspace::ResourceDescriptor,
    },
};

pub struct S3BucketDetails {}

impl S3BucketDetails {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn load(rd: &ResourceDescriptor) -> Result<Resource, Error> {
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

        Ok(Resource::S3Bucket(BucketInfo(bucket.to_owned())))
    }

    pub async fn list(profile: String, region: String) -> Result<Vec<Resource>, Error> {
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
            .map(|b| Resource::S3Bucket(BucketInfo(b)))
            .collect())
    }

    pub fn render(&self, b: &BucketInfo) -> Element<Message> {
        let mut c = column![];
        c = c.push(text("S3 Bucket Details").size(24));
        c = c.push(text(format!(
            "Name: {}",
            b.0.name.as_ref().unwrap_or(&"Unnamed".to_string())
        )));
        c.into()
    }
}
