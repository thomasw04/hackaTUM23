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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]
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
impl Eq for PostcodeInfo {}

impl Ord for PostcodeInfo {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.zipcode.cmp(&other.zipcode)
    }
}

impl std::hash::Hash for PostcodeInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.zipcode.hash(state);
    }
}

pub fn postcodes_from_file(path: &str) -> Result<Vec<PostcodeInfo>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let postcodes: Vec<PostcodeInfo> = serde_json::from_str(&file_content)?;
    Ok(postcodes)
}

use simsearch::SimSearch;

pub fn build_engine(postcodes: &Vec<PostcodeInfo>) -> SimSearch<PostcodeInfo> {
    let mut engine: SimSearch<PostcodeInfo> = SimSearch::new();

    for info in postcodes {
        let search_str = info.zipcode.to_string() + " " + &info.place;
        engine.insert(info.clone(), &search_str);
    }

    engine
}

