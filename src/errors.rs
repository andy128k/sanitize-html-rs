//! Error types, which can be emited by sanitization procedure.

use std::error::Error;
use std::fmt;

/// Sanitization error
#[derive(Debug)]
pub enum SanitizeError {
    /// UTF-8 decoding error
    Utf8Error(std::string::FromUtf8Error),

    /// Serialization error
    SerializeError(std::io::Error),
}

impl fmt::Display for SanitizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SanitizeError::Utf8Error(e) => write!(f, "UTF-8 decode error {}", e),
            SanitizeError::SerializeError(e) => write!(f, "Serialization error {}", e),
        }
    }
}

impl Error for SanitizeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            SanitizeError::Utf8Error(e) => Some(e),
            SanitizeError::SerializeError(e) => Some(e),
        }
    }
}
