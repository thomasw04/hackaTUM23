use std::fs;
use serde_json;
use std::error::Error;

use serde::{Deserialize, Serialize};
use std::str::FromStr;

fn from_str_u32<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u32::from_str(&s).map_err(serde::de::Error::custom)
}

fn from_str_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f32::from_str(&s).map_err(serde::de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostcodeInfo {
    #[serde(deserialize_with = "from_str_u32")]
    zipcode: u32,
    // Friendly display name (e.g. "Garching bei München")
    place: String,
    #[serde(deserialize_with = "from_str_f32")]
    latitude: f32,
    #[serde(deserialize_with = "from_str_f32")]
    longitude: f32,

    // There are other attributes we might want to use later, but don't need yet
}

pub fn postcodes_from_file(path: &str) -> Result<Vec<PostcodeInfo>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let postcodes: Vec<PostcodeInfo> = serde_json::from_str(&file_content)?;
    Ok(postcodes)
}

