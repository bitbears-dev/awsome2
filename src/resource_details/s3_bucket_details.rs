use iced::{
    widget::{column, text},
    Element,
};

use crate::{message::Message, resource::BucketInfo};

pub struct S3BucketDetails {}

impl S3BucketDetails {
    pub fn new() -> Self {
        Self {}
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
