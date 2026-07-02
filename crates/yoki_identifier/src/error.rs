use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdentifierError {
    #[error("identifier must not be empty")]
    Empty,
    #[error("invalid namespace `{namespace}`: must match [a-z0-9.-_]")]
    InvalidNamespace { namespace: String },
    #[error("invalid path `{path}`: must match [a-z0-9.-_/]")]
    InvalidPath { path: String },
}
