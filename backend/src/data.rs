use std::{fs, collections::{HashMap, HashSet}};
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

fn from_str_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    u8::from_str(&s).map_err(serde::de::Error::custom)
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Postcode {
    #[serde(deserialize_with = "from_str_u32")]
    postcode: u32,
    lon: f64,
    lat: f64,
    #[serde(deserialize_with = "from_str_u8")]
    postcode_extension_distance_group: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceProvider {
    id: u32,
    first_name: String,
    last_name: String,
    city: String,
    street: String,
    #[serde(deserialize_with = "from_str_u32")]
    house_number: u32,
    lon: f64,
    lat: f64,
    max_driving_distance: u32
}

const INITIAL_POSTCODE_DATA: &'static str = include_str!("../data/postcode.json");
const INITIAL_SERVICE_PROVIDER_DATA: &'static str = include_str!("../data/service_provider_profile.json");

pub fn postcode_from_file() -> Result<HashMap<u32, Postcode>, &'static str> {
    if let Ok(service_provider) = serde_json::from_str::<Vec<Postcode>>(INITIAL_POSTCODE_DATA) {
        Ok(service_provider.iter().map(|x| (x.postcode, (*x).to_owned())).collect())
    } else {
        Err("Failed to load postcode data.")
    }
}

pub fn provider_from_file() -> Result<HashMap<u32, ServiceProvider>, &'static str> {
    if let Ok(service_providers) = serde_json::from_str::<Vec<ServiceProvider>>(INITIAL_SERVICE_PROVIDER_DATA) {
        Ok(service_providers.iter().map(|x| (x.id, (*x).to_owned())).collect())
    } else {
        Err("Failed to load service providers.")
    }
}

pub fn postcode_info_from_file(path: &str) -> Result<Vec<PostcodeInfo>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let postcodes: Vec<PostcodeInfo> = serde_json::from_str(&file_content)?;
    Ok(postcodes)
}

