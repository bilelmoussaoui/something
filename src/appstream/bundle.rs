use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct Bundle {
    #[serde(rename = "type", default)]
    _type: String,
    runtime: Option<String>,
    sdk: String,
    #[serde(rename = "$value", default)]
    name: String,
}
