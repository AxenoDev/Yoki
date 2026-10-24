use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum RegistryError {
    #[error("could not find data/generated directory")]
    MissingGeneratedData,
    #[error("no registry versions found in {0}")]
    NoVersions(PathBuf),
    #[error("io error while reading {0}: {1}")]
    Io(String, #[source] std::io::Error),
    #[error("json error in {0}: {1}")]
    Json(String, #[source] serde_json::Error),
    #[error("missing registry {0}")]
    MissingRegistry(String),
    #[error("missing registry directory for {0}")]
    MissingRegistryDir(String),
    #[error("empty registry {0}")]
    EmptyRegistry(String),
    #[error("missing protocol id for {0}")]
    MissingProtocolId(String),
    #[error("invalid registry entry {0}")]
    InvalidEntry(String),
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
    #[error("{0} cannot be converted to NBT")]
    InvalidNbt(String),
    #[error("mandatory registry {registry} is missing for {version}")]
    MandatoryRegistryMissing { registry: String, version: String },
}

impl From<std::io::Error> for RegistryError {
    fn from(err: std::io::Error) -> Self {
        Self::Io("registry io".into(), err)
    }
}
