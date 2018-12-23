//! Error types, which can be emited by sanitization procedure.

use failure::Fail;

/// Sanitization error
#[derive(Fail, Debug)]
pub enum SanitizeError {
    /// UTF-8 decoding error
    #[fail(display = "UTF-8 decode error {}", _0)]
    Utf8Error(#[cause] std::string::FromUtf8Error),

    /// Serialization error
    #[fail(display = "Serialization error {}", _0)]
    SerializeError(#[cause] std::io::Error),
}
