//! Structures to define sanitization rules.

pub mod pattern;
pub mod predefined;

use self::pattern::Pattern;
use std::collections::HashMap;
use std::collections::HashSet;

/// structure to describe HTML element
pub struct Element {
    /// name of an element
    pub name: String,
    /// Whitelist of allowed attributes
    pub attributes: HashMap<String, Pattern>,
    /// List of mandatory atributes and their values.
    /// These attributes will be forcibly added to element.
    pub mandatory_attributes: HashMap<String, String>,
}

impl Element {
    /// Creates element descriptor
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            attributes: HashMap::new(),
            mandatory_attributes: HashMap::new(),
        }
    }

    /// Adds an attribute
    pub fn attribute(mut self, attribute: &str, pattern: Pattern) -> Self {
        self.attributes.insert(attribute.to_owned(), pattern);
        self
    }

    /// Adds mandatory attribute
    pub fn mandatory_attribute(mut self, attribute: &str, value: &str) -> Self {
        self.mandatory_attributes
            .insert(attribute.to_owned(), value.to_owned());
        self
    }

    /// Checks if attribute is valid
    pub fn is_valid(&self, attribute: &str, value: &str) -> bool {
        match self.attributes.get(attribute) {
            None => false,
            Some(pattern) => pattern.matches(value),
        }
    }
}

/// structure to describe sanitization rules
#[derive(Default)]
pub struct Rules {
    /// Determines if comments are kept of stripped out of a document.
    pub allow_comments: bool,
    /// Allowed elements.
    pub allowed_elements: HashMap<String, Element>,
    /// Elements which will be removed together with their children.
    pub delete_elements: HashSet<String>,
    /// Elements which will be replaced by spaces (Their children will be processed recursively).
    pub space_elements: HashSet<String>,
    /// Elements which will be renamed.
    pub rename_elements: HashMap<String, String>,
}

impl Rules {
    /// Creates a new rules set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets if comments are allowed
    pub fn allow_comments(mut self, allow_comments: bool) -> Self {
        self.allow_comments = allow_comments;
        self
    }

    /// Adds a rule for an allowed element
    pub fn element(mut self, element: Element) -> Self {
        self.allowed_elements.insert(element.name.clone(), element);
        self
    }

    /// Adds a rule to delete an element
    pub fn delete(mut self, element_name: &str) -> Self {
        self.delete_elements.insert(element_name.to_owned());
        self
    }

    /// Adds a rule to replace an element with space
    pub fn space(mut self, element_name: &str) -> Self {
        self.space_elements.insert(element_name.to_owned());
        self
    }

    /// Adds a rule to rename an element
    pub fn rename(mut self, element_name: &str, to: &str) -> Self {
        self.rename_elements
            .insert(element_name.to_owned(), to.to_owned());
        self
    }
}
