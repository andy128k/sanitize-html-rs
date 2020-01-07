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
pub mod rules;
mod sanitize;
mod tests;

use crate::errors::SanitizeError;
use crate::rules::Rules;

/// Sanitize HTML bytes
pub fn sanitize_bytes(rules: &Rules, input: &[u8]) -> Result<Vec<u8>, SanitizeError> {
    let input_str = std::str::from_utf8(input).map_err(SanitizeError::StrUtf8Error)?;
    let dom = parse::parse_str(input_str);
    let new_dom = sanitize::sanitize_dom(&dom, rules);
    let result_bytes = parse::unparse_bytes(&new_dom)?;
    Ok(result_bytes)
}

/// Sanitize HTML string
pub fn sanitize_str(rules: &Rules, input: &str) -> Result<String, SanitizeError> {
    let dom = parse::parse_str(input);
    let new_dom = sanitize::sanitize_dom(&dom, rules);
    let result_bytes = parse::unparse_bytes(&new_dom)?;
    let result_string = String::from_utf8(result_bytes).map_err(SanitizeError::Utf8Error)?;
    Ok(result_string)
}
