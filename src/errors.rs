//! Errors, which can be emited by sanitization procedure.

use std::error::Error;
use std::fmt;

/// Sanitization error
#[derive(Debug)]
pub struct SanitizeError(pub(crate) Box<dyn Error>);

impl fmt::Display for SanitizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for SanitizeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.0.source()
    }
}
