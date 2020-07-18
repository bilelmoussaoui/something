use serde::de;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Screenshot {
    #[serde(
        rename = "type",
        deserialize_with = "screenshot_type_deserialize",
        default
    )]
    is_default: bool,
    #[serde(rename = "image", default)]
    images: Vec<Image>,
}

fn screenshot_type_deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;

    Ok(s == "default")
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Image {
    #[serde(rename = "type", deserialize_with = "image_type_deserialize", default)]
    _type: ImageType,
    width: Option<u32>,
    height: Option<u32>,
    #[serde(rename = "$value", default)]
    url: String,
}

fn image_type_deserialize<'de, D>(deserializer: D) -> Result<ImageType, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(ImageType::from_str(&s).unwrap())
}

#[derive(Debug, Serialize, PartialEq)]
pub enum ImageType {
    Source,
    Thumbnail,
}
impl Default for ImageType {
    fn default() -> Self {
        ImageType::Source
    }
}

impl FromStr for ImageType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "source" => Ok(ImageType::Source),
            "thumbnail" => Ok(ImageType::Thumbnail),
            _ => anyhow::bail!("Unsupported image type"),
        }
    }
}
