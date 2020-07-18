use serde::de;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

fn icon_type_deserialize<'de, D>(deserializer: D) -> Result<IconType, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(IconType::from_str(&s).unwrap())
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Icon {
    #[serde(rename = "type", deserialize_with = "icon_type_deserialize", default)]
    _type: IconType,
    height: Option<u32>,
    width: Option<u32>,
    #[serde(rename = "$value")]
    name: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub enum IconType {
    Cached,
    Remote,
    Stock,
    Unknown,
}

impl Default for IconType {
    fn default() -> Self {
        IconType::Unknown
    }
}

impl FromStr for IconType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cached" => Ok(IconType::Cached),
            "remote" => Ok(IconType::Remote),
            "stock" => Ok(IconType::Stock),
            e => anyhow::bail!("Failed to parse the icon type {}", e),
        }
    }
}
