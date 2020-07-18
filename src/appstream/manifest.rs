use super::Component;
use anyhow::Result;
use flate2::read::GzDecoder;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Manifest {
    version: String,
    origin: String,
    #[serde(rename = "component", default)]
    pub components: Vec<Component>,
}

impl Manifest {
    pub fn from_path(path: PathBuf) -> Result<Self> {
        let xml = std::fs::read_to_string(path)?;

        let manifest: Manifest = from_str(&xml)?;
        Ok(manifest)
    }

    pub fn from_gzipped(path: PathBuf) -> Result<Self> {
        let f = File::open(path)?;

        let mut d = GzDecoder::new(f);
        let mut xml = String::new();
        d.read_to_string(&mut xml)?;

        let manifest: Manifest = from_str(&xml)?;
        Ok(manifest)
    }
}
