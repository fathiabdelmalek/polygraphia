use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum PolygraphiaError {
    InvalidKey(String),
    InvalidInput(String),
    EncryptionError(String),
    DecryptionError(String),
}

impl fmt::Display for PolygraphiaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolygraphiaError::InvalidKey(msg) => write!(f, "Invalid Key: {}", msg),
            PolygraphiaError::InvalidInput(msg) => write!(f, "Invalid Input: {}", msg),
            PolygraphiaError::EncryptionError(msg) => write!(f, "Encryption Error: {}", msg),
            PolygraphiaError::DecryptionError(msg) => write!(f, "Decryption Error: {}", msg),
        }
    }
}

impl std::error::Error for PolygraphiaError {}
