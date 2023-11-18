use std::{fs, collections::HashMap};
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

fn group_str_to_u8<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    // either group_a, group_b, group_c
    match s.as_str() {
        "group_a" => Ok(0),
        "group_b" => Ok(1),
        "group_c" => Ok(2),
        _ => Err(serde::de::Error::custom("Invalid postcode extension distance group."))
    }
}

fn from_str_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    f32::from_str(&s).map_err(serde::de::Error::custom)
}

fn to_radians<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    f64::deserialize(deserializer).map(|x| x.to_radians())
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, PartialOrd)]pub struct PostcodeInfo {
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

use simsearch::SimSearch;

pub fn build_engine(postcodes: &Vec<PostcodeInfo>) -> SimSearch<PostcodeInfo> {
    let mut engine: SimSearch<PostcodeInfo> = SimSearch::new();

    for info in postcodes {
        let search_str = info.zipcode.to_string() + " " + &info.place;
        engine.insert(info.clone(), &search_str);
    }

    engine
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Postcode {
    #[serde(deserialize_with = "from_str_u32")]
    pub postcode: u32,

    #[serde(deserialize_with = "to_radians")]
    pub lon: f64,

    #[serde(deserialize_with = "to_radians")]
    pub lat: f64,

    #[serde(deserialize_with = "group_str_to_u8")]
    pub postcode_extension_distance_group: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ServiceProvider {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub city: String,
    pub street: String,
    pub house_number: String,

    #[serde(deserialize_with = "to_radians")]
    pub lon: f64,

    #[serde(deserialize_with = "to_radians")]
    pub lat: f64,

    pub max_driving_distance: u64
}

const INITIAL_POSTCODE_DATA: &'static str = include_str!("../data/postcode.json");
const INITIAL_SERVICE_PROVIDER_DATA: &'static str = include_str!("../data/service_provider_profile.json");

pub fn postcode_from_file() -> Result<HashMap<u32, Postcode>, String> {
    let res = serde_json::from_str::<Vec<Postcode>>(INITIAL_POSTCODE_DATA);

    if let Ok(service_provider) = res  {
        Ok(service_provider.iter().map(|x| (x.postcode, (*x).to_owned())).collect())
    } else if let Err(e) = res {
        Err(format!("{e}").to_string())
    } else {
        Err("Failed to parse postcode. Unknown error.".to_string())
    }
}

pub fn provider_from_file() -> Result<HashMap<u32, ServiceProvider>, String> {
    let res = serde_json::from_str::<Vec<ServiceProvider>>(INITIAL_SERVICE_PROVIDER_DATA);

    if let Ok(service_providers) = serde_json::from_str::<Vec<ServiceProvider>>(INITIAL_SERVICE_PROVIDER_DATA) {
        Ok(service_providers.iter().map(|x| (x.id, (*x).to_owned())).collect())
    } else if let Err(e) = res  {
        Err(format!("{e}").to_string())
    } else {
        Err("Failed to load service providers. Unknown error.".to_string())
    }
}

pub fn postcode_info_from_file(path: &str) -> Result<Vec<PostcodeInfo>, Box<dyn Error>> {
    let file_content = fs::read_to_string(path)?;
    let postcodes: Vec<PostcodeInfo> = serde_json::from_str(&file_content)?;
    Ok(postcodes)
}

