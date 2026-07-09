use std::fmt;

#[derive(Debug)]
pub struct CommandError(String);

impl CommandError {
    pub fn msg(message: impl Into<String>) -> Self {
        Self(message.into())
    }

    pub fn message(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ArgError {
    Missing(&'static str),
    Invalid { name: &'static str, reason: String },
    TooMany,
}

impl fmt::Display for ArgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Missing(name) => write!(f, "missing argument <{name}>"),
            Self::Invalid { name, reason } => write!(f, "invalid argument <{name}>: {reason}"),
            Self::TooMany => f.write_str("too many arguments"),
        }
    }
}
