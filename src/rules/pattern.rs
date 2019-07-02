//! This module contains code dedicated to check validity of attribute's value.
//! 
//! # Examples
//! ```
//! # extern crate sanitize_html;
//! # extern crate regex;
//! # fn main() {
//! use sanitize_html::rules::pattern::Pattern;
//! use regex::Regex;
//!
//! let href = Pattern::regex(Regex::new("^(ftp:|http:|https:|mailto:)").unwrap()) |
//!     !Pattern::regex(Regex::new("^[^/]+[[:space:]]*:").unwrap());
//!
//! assert!(href.matches("filename.xls"));
//! assert!(href.matches("http://foo.com/"));
//! assert!(href.matches(" filename with spaces .zip "));
//! assert!(!href.matches(" javascript  : window.location = '//example.com/'")); // Attempt to make XSS
//! # }
//! ```
//!

use regex::Regex;

/// Value pattern
pub struct Pattern(pub Box<dyn Fn(&str) -> bool + Sync + Send>);

impl Pattern {
    /// Creates pattern which accepts any value.
    /// 
    /// # Example
    /// ```
    /// # extern crate sanitize_html;
    /// # extern crate regex;
    /// use sanitize_html::rules::pattern::Pattern;
    /// # fn main() {
    /// use regex::Regex;
    ///
    /// let pattern = Pattern::any();
    /// assert!(pattern.matches(""));
    /// assert!(pattern.matches("pants"));
    /// # }
    /// ```
    pub fn any() -> Self {
        Pattern(Box::new(move |_value| true))
    }

    /// Creates pattern which uses regular expression to check a value. Panics
    /// 
    /// # Example
    /// ```
    /// # extern crate sanitize_html;
    /// # extern crate regex;
    /// use sanitize_html::rules::pattern::Pattern;
    /// # fn main() {
    /// use regex::Regex;
    ///
    /// let pattern = Pattern::regex(Regex::new("ant").unwrap());
    /// assert!(!pattern.matches(""));
    /// assert!(pattern.matches("pants"));
    /// # }
    /// ```
    pub fn regex(re: Regex) -> Self {
        Pattern(Box::new(move |value| re.is_match(value)))
    }

    /// Checks if a value matches to a pattern.
    pub fn matches(&self, value: &str) -> bool {
        (self.0)(value)
    }
}

impl ::std::ops::Not for Pattern {
    type Output = Pattern;

    /// Negates pattern
    /// 
    /// # Example
    /// ```
    /// # extern crate sanitize_html;
    /// # extern crate regex;
    /// use sanitize_html::rules::pattern::Pattern;
    /// # fn main() {
    /// use regex::Regex;
    ///
    /// let pattern = !Pattern::any();
    /// assert!(!pattern.matches(""));
    /// assert!(!pattern.matches("pants"));
    /// # }
    /// ```
    fn not(self) -> Self::Output {
        let cb = self.0;
        Pattern(Box::new(move |value| !cb(value)))
    }
}

impl ::std::ops::BitAnd for Pattern {
    type Output = Pattern;

    /// Combines two patterns into a pattern which matches a string iff both patterns match that string.
    /// 
    /// # Example
    /// ```
    /// # extern crate sanitize_html;
    /// # extern crate regex;
    /// use sanitize_html::rules::pattern::Pattern;
    /// # fn main() {
    /// use regex::Regex;
    ///
    /// let pan = Pattern::regex(Regex::new("pan").unwrap());
    /// let ant = Pattern::regex(Regex::new("ant").unwrap());
    /// let pattern = pan & ant;
    /// 
    /// assert!(!pattern.matches("pan"));
    /// assert!(!pattern.matches("ant"));
    /// assert!(pattern.matches("pants"));
    /// # }
    /// ```
    fn bitand(self, rhs: Pattern) -> Self::Output {
        let cb1 = self.0;
        let cb2 = rhs.0;
        Pattern(Box::new(move |value| cb1(value) && cb2(value)))
    }
}

impl ::std::ops::BitOr for Pattern {
    type Output = Pattern;

    /// Combines two patterns into a pattern which matches a string if one of patterns matches that string.
    /// 
    /// # Example
    /// ```
    /// # extern crate sanitize_html;
    /// # extern crate regex;
    /// use sanitize_html::rules::pattern::Pattern;
    /// # fn main() {
    /// use regex::Regex;
    ///
    /// let pan = Pattern::regex(Regex::new("pan").unwrap());
    /// let pot = Pattern::regex(Regex::new("pot").unwrap());
    /// let pattern = pan | pot;
    /// 
    /// assert!(pattern.matches("pants"));
    /// assert!(pattern.matches("pot"));
    /// assert!(!pattern.matches("jar"));
    /// # }
    /// ```
    fn bitor(self, rhs: Pattern) -> Self::Output {
        let cb1 = self.0;
        let cb2 = rhs.0;
        Pattern(Box::new(move |value| cb1(value) || cb2(value)))
    }
}
