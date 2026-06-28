use std::fmt;
use std::str::FromStr;

use crate::error::IdentifierError;

pub const MINECRAFT_NAMESPACE: &str = "minecraft";

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Identifier {
    namespace: String,
    path: String,
}

impl Identifier {
    pub fn new(
        namespace: impl Into<String>,
        path: impl Into<String>,
    ) -> Result<Self, IdentifierError> {
        let namespace = namespace.into();
        let path = path.into();

        if !is_valid_namespace(&namespace) {
            return Err(IdentifierError::InvalidNamespace { namespace });
        }

        if !is_valid_path(&path) {
            return Err(IdentifierError::InvalidPath { path });
        }

        Ok(Self { namespace, path })
    }

    pub fn minecraft(path: impl Into<String>) -> Result<Self, IdentifierError> {
        Self::new(MINECRAFT_NAMESPACE, path)
    }

    pub fn parse(input: &str) -> Result<Self, IdentifierError> {
        if input.is_empty() {
            return Err(IdentifierError::Empty);
        }

        match input.split_once(':') {
            Some((namespace, path)) => Self::new(namespace, path),
            None => Self::minecraft(input),
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn path(&self) -> &str {
        &self.path
    }

    pub fn as_str(&self) -> String {
        format!("{}:{}", self.namespace, self.path)
    }

    pub fn is_minecraft(&self) -> bool {
        self.namespace == MINECRAFT_NAMESPACE
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.namespace, self.path)
    }
}

impl FromStr for Identifier {
    type Err = IdentifierError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Identifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = <String as serde::Deserialize>::deserialize(deserializer)?;
        Self::parse(&value).map_err(serde::de::Error::custom)
    }
}

fn is_valid_namespace(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(is_namespace_byte)
}

fn is_valid_path(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(is_path_byte)
}

const fn is_namespace_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_'
    )
}

const fn is_path_byte(byte: u8) -> bool {
    matches!(
        byte,
        b'a'..=b'z' | b'0'..=b'9' | b'.' | b'-' | b'_' | b'/'
    )
}
