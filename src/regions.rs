use aws_config::{BehaviorVersion, Region};

use crate::error::Error;

pub async fn load_regions(profile: String, nearest_region: String) -> Result<Vec<String>, Error> {
    let config = aws_config::defaults(BehaviorVersion::v2024_03_28())
        .profile_name(profile)
        .region(Region::new(nearest_region.to_string()))
        .load()
        .await;
    let client = aws_sdk_ec2::Client::new(&config);
    let out = client.describe_regions().send().await?;
    let mut regions = out
        .regions
        .map(|regions| {
            regions
                .iter()
                .filter_map(|region| region.region_name.clone())
                .collect()
        })
        .unwrap_or_else(|| {
            eprintln!("Regions was None");
            vec![]
        });

    regions.sort();

    Ok(regions)
}
