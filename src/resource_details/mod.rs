pub mod lambda_function_details;
pub mod s3_bucket_details;

use iced::{
    widget::{container, text},
    Element,
};

use crate::{message::Message, resource::Resource};

use lambda_function_details::LambdaFunctionDetails;
use s3_bucket_details::S3BucketDetails;

pub struct ResourceDetails {
    resource: Option<Resource>,
    lambda_function_details: LambdaFunctionDetails,
    s3_bucket_details: S3BucketDetails,
}

impl ResourceDetails {
    pub fn new() -> Self {
        Self {
            resource: None,
            lambda_function_details: LambdaFunctionDetails::new(),
            s3_bucket_details: S3BucketDetails::new(),
        }
    }

    pub fn set_resource(&mut self, resource: Option<Resource>) {
        self.resource = resource;
    }

    pub fn view(&self) -> Element<Message> {
        match &self.resource {
            Some(resource) => match resource {
                Resource::LambdaFunction(f) => {
                    return self.lambda_function_details.render(f);
                }
                Resource::S3Bucket(b) => {
                    return self.s3_bucket_details.render(b);
                }
            },
            None => container(text("No resource selected")).into(),
        }
    }
}
