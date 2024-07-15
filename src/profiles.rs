use std::path::PathBuf;
use std::sync::Mutex;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::Error;

static PROFILE_PATTERN_IN_AWS_CONFIG: Lazy<Mutex<Regex>> = Lazy::new(|| {
    let re = Regex::new(r#"^\[profile\s+(.+)\]"#).unwrap();
    Mutex::new(re)
});

static PROFILE_PATTERN_IN_AWS_CREDENTIALS: Lazy<Mutex<Regex>> = Lazy::new(|| {
    let re = Regex::new(r#"^\[(.+)\]"#).unwrap();
    Mutex::new(re)
});

pub fn load_profiles() -> Result<Vec<String>, Error> {
    let aws_config_path = get_aws_config_path()?;
    let aws_config_file_contents = std::fs::read_to_string(aws_config_path)?;
    let mut profiles_in_aws_config = collect_profiles_from_aws_config(&aws_config_file_contents)?;

    let aws_credentials_path = get_aws_credentials_path()?;
    let aws_credentials_file_contents = std::fs::read_to_string(aws_credentials_path)?;
    let mut profiles_in_aws_credentials =
        collect_profiles_from_aws_credentials(&aws_credentials_file_contents)?;

    profiles_in_aws_config.append(&mut profiles_in_aws_credentials);

    Ok(profiles_in_aws_config)
}

fn get_aws_config_path() -> Result<PathBuf, Error> {
    let Some(user_dirs) = directories::UserDirs::new() else {
        return Err(Error::UnableToLoadAwsConfig);
    };

    let mut path = PathBuf::new();
    path.push(user_dirs.home_dir());
    path.push(".aws");
    path.push("config");
    Ok(path)
}

fn get_aws_credentials_path() -> Result<PathBuf, Error> {
    let Some(user_dirs) = directories::UserDirs::new() else {
        return Err(Error::UnableToLoadAwsConfig);
    };

    let mut path = PathBuf::new();
    path.push(user_dirs.home_dir());
    path.push(".aws");
    path.push("credentials");
    Ok(path)
}

// Contents of ~/.aws/config is look like:
//
// ```
// [profile dev]
// services = my-services
//
// [services my-services]
// dynamodb =
//   endpoint_url = http://localhost:8000`
// ```
//
// it may have indented "sub-sections". It is different from .ini or .toml.
// So we cannot use any existing crates. We should implement our own function to parse it.
fn collect_profiles_from_aws_config(content: &str) -> Result<Vec<String>, Error> {
    let mut result = vec![];

    for line in content.lines() {
        if let Some(captures) = PROFILE_PATTERN_IN_AWS_CONFIG.lock().unwrap().captures(line) {
            if captures.len() >= 2 {
                if let Some(cap) = captures.get(1) {
                    result.push(cap.as_str().to_string());
                }
            }
        }
    }
    Ok(result)
}

fn collect_profiles_from_aws_credentials(content: &str) -> Result<Vec<String>, Error> {
    let mut result = vec![];

    for line in content.lines() {
        if let Some(captures) = PROFILE_PATTERN_IN_AWS_CREDENTIALS
            .lock()
            .unwrap()
            .captures(line)
        {
            if captures.len() >= 2 {
                if let Some(cap) = captures.get(1) {
                    result.push(cap.as_str().to_string());
                }
            }
        }
    }
    Ok(result)
}
