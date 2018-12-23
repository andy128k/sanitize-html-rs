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
//! assert_eq!(&sanitized_default, "Lorem ipsum dolor sit amet alert(\"hello world\");");
//! ```

#![deny(missing_docs)]

pub mod errors;
pub mod rules;
mod sanitize;
mod parse;
mod tests;

use crate::errors::SanitizeError;
use crate::rules::Rules;

/// Sanitize HTML bytes
pub fn sanitize_bytes(rules: &Rules, input: &[u8]) -> Result<Vec<u8>, SanitizeError> {
    let mut dom = parse::parse_bytes(input);
    sanitize::sanitize_dom(&mut dom, rules);
    let buf = parse::unparse_bytes(dom)?;
    Ok(buf)
}

/// Sanitize HTML string
pub fn sanitize_str(rules: &Rules, input: &str) -> Result<String, SanitizeError> {
    let result_bytes = sanitize_bytes(rules, input.as_bytes())?;
    let result_string = String::from_utf8(result_bytes).map_err(SanitizeError::Utf8Error)?;
    Ok(result_string)
}
