use chrono::{DateTime, TimeZone, Utc};
use serde::de;
use serde::{Deserialize, Serialize};

fn timestamp_deserialize<'de, D>(
    deserializer: D,
) -> Result<Option<chrono::DateTime<chrono::Utc>>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s = String::deserialize(deserializer);
    match s {
        Ok(timestamp) => Ok(Some(
            chrono::Utc
                .datetime_from_str(&timestamp, "%s")
                .map_err(serde::de::Error::custom)?,
        )),
        Err(_) => Ok(None),
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Release {
    #[serde(deserialize_with = "timestamp_deserialize", default)]
    timestamp: Option<DateTime<Utc>>,
    version: String,
}
