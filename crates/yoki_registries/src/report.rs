use std::{collections::HashMap, fs::read_to_string, path::Path};

use serde::Deserialize;

use crate::error::RegistryError;

#[derive(Debug, Deserialize)]
pub struct DatapackReport {
    pub registries: HashMap<String, DatapackRegistryInfo>,
}

#[derive(Debug, Deserialize)]
pub struct DatapackRegistryInfo {
    pub elements: bool,
}

#[derive(Debug, Deserialize)]
pub struct RegistriesReport {
    #[serde(flatten)]
    pub registries: HashMap<String, RegistryReport>,
}

#[derive(Debug, Deserialize)]
pub struct RegistryReport {
    pub entries: HashMap<String, EntryReport>,
}

#[derive(Debug, Deserialize)]
pub struct EntryReport {
    pub protocol_id: i32,
}

pub fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, RegistryError> {
    let raw =
        read_to_string(path).map_err(|err| RegistryError::Io(path.display().to_string(), err))?;
    serde_json::from_str(&raw).map_err(|err| RegistryError::Json(path.display().to_string(), err))
}
