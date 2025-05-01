//! HTML Sanitization library
//!
//! # Examples
//!
//! ```
//! use sanitize_html::sanitize_str;
//! use sanitize_html::rules::predefined::DEFAULT;
//!
//! let input = "<b>Lo<!-- comment -->rem</b> <a href=\"pants\" title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br/>amet <script>alert(\"hello world\");</script>";
//!
//! let sanitized_default: String = sanitize_str(&DEFAULT, input).unwrap();
//! assert_eq!(&sanitized_default, "Lorem ipsum dolor sit amet ");
//! ```

#![deny(missing_docs)]

pub mod errors;
mod parse;
mod rcdom;
pub mod rules;
mod sanitize;
mod tests;

use crate::errors::SanitizeError;
use crate::rules::Rules;
use std::error::Error;

/// Sanitize HTML bytes
pub fn sanitize_bytes(rules: &Rules, input: &[u8]) -> Result<Vec<u8>, SanitizeError> {
    fn inner(rules: &Rules, input: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let dom = parse::parse_dom(input)?;
        let new_document = sanitize::sanitize_dom(&dom, rules);
        let result_bytes = parse::unparse_document(&new_document)?;
        Ok(result_bytes)
    }
    inner(rules, input).map_err(SanitizeError)
}

/// Sanitize HTML string
pub fn sanitize_str(rules: &Rules, input: &str) -> Result<String, SanitizeError> {
    let result_bytes = sanitize_bytes(rules, input.as_bytes())?;
    let result_string = String::from_utf8(result_bytes).map_err(|e| SanitizeError(Box::new(e)))?;
    Ok(result_string)
}
