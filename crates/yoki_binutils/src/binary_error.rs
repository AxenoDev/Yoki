use std::io;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("unexpected eof")]
    UnexpectedEof,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    InvalidUtf8(#[from] FromUtf8Error),
    #[error("var int too big")]
    VarIntTooBig,
    #[error("var long too big")]
    VarLongTooBig,
    #[error("invalid uuid")]
    InvalidUuid,
    #[error("invalid identifier: {0}")]
    InvalidIdentifier(String),
}

impl BinaryError {
    pub fn from_io(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::UnexpectedEof => Self::UnexpectedEof,
            _ => Self::Io(err),
        }
    }
}
