extern crate regex;
#[macro_use] extern crate html5ever;

pub mod rules;
mod sanitize;
mod parse;
#[cfg(test)] mod tests;

use std::error;
use rules::Rules;

type StdResult<T> = Result<T, Box<error::Error>>;

pub fn sanitize_bytes(rules: &Rules, input: &[u8]) -> StdResult<Vec<u8>> {
    let mut dom = parse::parse_bytes(input);
    sanitize::sanitize_dom(&mut dom, rules);
    let buf = parse::unparse_bytes(dom)?;
    Ok(buf)
}

pub fn sanitize_str(rules: &Rules, input: &str) -> StdResult<String> {
    let result_bytes = sanitize_bytes(rules, input.as_bytes())?;
    let result_string = String::from_utf8(result_bytes)?;
    Ok(result_string)
}
