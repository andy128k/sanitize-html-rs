pub mod predefined;

use std::error;
use std::collections::HashMap;
use std::collections::HashSet;
use regex::Regex;

type StdResult<T> = Result<T, Box<error::Error>>;

pub struct Attribute {
    pub name: String,
    re: Option<Regex>,
    re_inv: Option<Regex>,
}

impl Attribute {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            re: None,
            re_inv: None,
        }
    }

    pub fn should_match(mut self, re: &str) -> StdResult<Self> {
        self.re = Some(Regex::new(re)?);
        Ok(self)
    }

    pub fn should_not_match(mut self, re: &str) -> StdResult<Self> {
        self.re_inv = Some(Regex::new(re)?);
        Ok(self)
    }

    pub fn is_valid(&self, value: &str) -> bool {
        if self.re.is_none() && self.re_inv.is_none() {
            return true;
        }
        if let Some(ref re) = self.re {
            if re.is_match(value) {
                return true;
            }
        }
        if let Some(ref re) = self.re_inv {
            if !re.is_match(value) {
                return true;
            }
        }
        false
    }
}

pub struct Element {
    pub name: String,
    pub attributes: HashMap<String, Attribute>,
    pub mandatory_attributes: HashMap<String, String>,
}

impl Element {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            attributes: HashMap::new(),
            mandatory_attributes: HashMap::new(),
        }
    }

    pub fn attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.insert(attribute.name.clone(), attribute);
        self
    }

    pub fn mandatory_attribute(mut self, attribute: &str, value: &str) -> Self {
        self.mandatory_attributes.insert(attribute.to_owned(), value.to_owned());
        self
    }

    pub fn is_valid(&self, attribute: &str, value: &str) -> bool {
        match self.attributes.get(attribute) {
            None => false,
            Some(attribute) => attribute.is_valid(value),
        }
    }
}

pub struct Rules {
    pub allow_comments: bool,
    pub allowed_elements: HashMap<String, Element>,
    pub delete_elements: HashSet<String>,
    pub space_elements: HashSet<String>,
    pub rename_elements: HashMap<String, String>,
}

impl Rules {
    pub fn new() -> Self {
        Self {
            allow_comments: false,
            allowed_elements: HashMap::new(),
            delete_elements: HashSet::new(),
            space_elements: HashSet::new(),
            rename_elements: HashMap::new(),
        }
    }

    pub fn allow_comments(mut self, allow_comments: bool) -> Self {
        self.allow_comments = allow_comments;
        self
    }

    pub fn element(mut self, element: Element) -> Self {
        self.allowed_elements.insert(element.name.clone(), element);
        self
    }

    pub fn delete(mut self, element_name: &str) -> Self {
        self.delete_elements.insert(element_name.to_owned());
        self
    }

    pub fn space(mut self, element_name: &str) -> Self {
        self.space_elements.insert(element_name.to_owned());
        self
    }

    pub fn rename(mut self, element_name: &str, to: &str) -> Self {
        self.rename_elements.insert(element_name.to_owned(), to.to_owned());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::Attribute;

    #[test]
    fn test_href() {
        let href = Attribute::new("href")
            .should_match("^(ftp:|http:|https:|mailto:)").unwrap()
            .should_not_match("^[^/]+[[:space:]]*:").unwrap()
        ;

        assert!(href.is_valid("pants"));
        assert!(href.is_valid("http://foo.com/"));
        assert!(href.is_valid(" pants "));
        assert!(!href.is_valid(" javascript  : window.location = '//example.com/'"));
    }
}
