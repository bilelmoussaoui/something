use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct Language {
    percentage: Option<u32>,
    #[serde(rename = "$value")]
    identifier: String,
}
