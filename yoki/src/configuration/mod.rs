use thiserror::Error;

pub mod server_properties;
pub mod toml_config;
pub mod yoki_config;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to read or write config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse server.properties: {0}")]
    Parse(#[from] serde_java_properties::de::Error),
    #[error("Failed to serialize server.properties: {0}")]
    Serialize(#[from] serde_java_properties::ser::Error),
    #[error("Failed to parse yoki.toml: {0}")]
    TomlParse(#[from] toml::de::Error),
    #[error("Failed to serialize yoki.toml: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("versioning.allowed in yoki.toml is empty: no client version could join")]
    NoAllowedVersion,
    #[error("Failed to load registry data: {0}")]
    Registry(String),
}
