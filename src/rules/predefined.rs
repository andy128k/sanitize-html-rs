//! Predefined rules
//!
//! These rules are inspired by a great Ruby gem [sanitize](https://github.com/rgrove/sanitize/).

use super::pattern::Pattern;
use super::{Element, Rules};
use lazy_static::lazy_static;
use regex::Regex;

fn re(regex: &str) -> Pattern {
    Pattern::regex(Regex::new(regex).unwrap())
}

fn href() -> Pattern {
    re("^(ftp:|http:|https:|mailto:)") | !re("^[^/]+[[:space:]]*:")
}

fn src() -> Pattern {
    re("^(http:|https:)") | !re("^[^/]+[[:space:]]*:")
}

lazy_static! {
    /// Basic rules. Allows a variety of markup including formatting elements, links, and lists.
    pub static ref BASIC: Rules = basic();

    /// Default rules. Removes all tags.
    pub static ref DEFAULT: Rules = default();

    /// Relaxed rules. Allows an even wider variety of markup, including images and tables
    pub static ref RELAXED: Rules = relaxed();

    /// Restricted rules. Allows only very simple inline markup. No links, images, or block elements.
    pub static ref RESTRICTED: Rules = restricted();

    /// Rules for document from untrusted sources. Removes all tags but text emphasizing and links.
    pub static ref UNTRUSTED: Rules = untrusted();
}

fn basic() -> Rules {
    Rules::new()
        .element(Element::new("a").attribute("href", href()))
        .element(Element::new("abbr").attribute("title", Pattern::any()))
        .element(Element::new("b"))
        .element(Element::new("blockquote").attribute("cite", src()))
        .element(Element::new("br"))
        .element(Element::new("br"))
        .element(Element::new("cite"))
        .element(Element::new("code"))
        .element(Element::new("dd"))
        .element(Element::new("dfn").attribute("title", Pattern::any()))
        .element(Element::new("dl"))
        .element(Element::new("dt"))
        .element(Element::new("em"))
        .element(Element::new("i"))
        .element(Element::new("kbd"))
        .element(Element::new("li"))
        .element(Element::new("mark"))
        .element(Element::new("ol"))
        .element(Element::new("p"))
        .element(Element::new("pre"))
        .element(Element::new("q").attribute("cite", src()))
        .element(Element::new("s"))
        .element(Element::new("samp"))
        .element(Element::new("small"))
        .element(Element::new("strike"))
        .element(Element::new("strong"))
        .element(Element::new("sub"))
        .element(Element::new("sup"))
        .element(
            Element::new("time")
                .attribute("datetime", Pattern::any())
                .attribute("pubdate", Pattern::any()),
        )
        .element(Element::new("u"))
        .element(Element::new("ul"))
        .element(Element::new("var"))
        .space("address")
        .space("article")
        .space("aside")
        .space("div")
        .space("footer")
        .space("h1")
        .space("h2")
        .space("h3")
        .space("h4")
        .space("h5")
        .space("h6")
        .space("header")
        .space("hgroup")
        .space("hr")
        .space("nav")
        .space("section")
}

fn default() -> Rules {
    Rules::new()
        .space("address")
        .space("article")
        .space("aside")
        .space("blockquote")
        .space("br")
        .space("dd")
        .space("div")
        .space("dl")
        .space("dt")
        .space("footer")
        .space("h1")
        .space("h2")
        .space("h3")
        .space("h4")
        .space("h5")
        .space("h6")
        .space("header")
        .space("hgroup")
        .space("hr")
        .space("li")
        .space("nav")
        .space("ol")
        .space("p")
        .space("pre")
        .space("section")
        .space("ul")
        .delete("iframe")
        .delete("noembed")
        .delete("noframes")
        .delete("noscript")
        .delete("script")
        .delete("style")
}

fn relaxed() -> Rules {
    fn relaxed_element(name: &str) -> Element {
        Element::new(name)
            .attribute("dir", Pattern::any())
            .attribute("lang", Pattern::any())
            .attribute("title", Pattern::any())
            .attribute("class", Pattern::any())
    }

    Rules::new()
        .element(relaxed_element("a").attribute("href", href()))
        .element(relaxed_element("abbr"))
        .element(relaxed_element("b"))
        .element(relaxed_element("bdo"))
        .element(relaxed_element("blockquote").attribute("cite", src()))
        .element(relaxed_element("br"))
        .element(relaxed_element("caption"))
        .element(relaxed_element("cite"))
        .element(relaxed_element("code"))
        .element(
            relaxed_element("col")
                .attribute("span", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(
            relaxed_element("colgroup")
                .attribute("span", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("dd"))
        .element(
            relaxed_element("del")
                .attribute("cite", src())
                .attribute("datetime", Pattern::any()),
        )
        .element(relaxed_element("dfn"))
        .element(relaxed_element("dl"))
        .element(relaxed_element("dt"))
        .element(relaxed_element("em"))
        .element(relaxed_element("figcaption"))
        .element(relaxed_element("figure"))
        .element(relaxed_element("h1"))
        .element(relaxed_element("h2"))
        .element(relaxed_element("h3"))
        .element(relaxed_element("h4"))
        .element(relaxed_element("h5"))
        .element(relaxed_element("h6"))
        .element(relaxed_element("hgroup"))
        .element(relaxed_element("i"))
        .element(
            relaxed_element("img")
                .attribute("src", src())
                .attribute("align", Pattern::any())
                .attribute("alt", Pattern::any())
                .attribute("width", Pattern::any())
                .attribute("height", Pattern::any()),
        )
        .element(
            relaxed_element("ins")
                .attribute("cite", src())
                .attribute("datetime", Pattern::any()),
        )
        .element(relaxed_element("kbd"))
        .element(relaxed_element("li"))
        .element(relaxed_element("mark"))
        .element(
            relaxed_element("ol")
                .attribute("start", Pattern::any())
                .attribute("reversed", Pattern::any())
                .attribute("type", Pattern::any()),
        )
        .element(relaxed_element("p"))
        .element(relaxed_element("pre"))
        .element(relaxed_element("q").attribute("cite", src()))
        .element(relaxed_element("rp"))
        .element(relaxed_element("rt"))
        .element(relaxed_element("ruby"))
        .element(relaxed_element("s"))
        .element(relaxed_element("samp"))
        .element(relaxed_element("small"))
        .element(relaxed_element("strike"))
        .element(relaxed_element("strong"))
        .element(relaxed_element("sub"))
        .element(relaxed_element("sup"))
        .element(
            relaxed_element("table")
                .attribute("summary", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("tbody"))
        .element(
            relaxed_element("td")
                .attribute("abbr", Pattern::any())
                .attribute("axis", Pattern::any())
                .attribute("colspan", Pattern::any())
                .attribute("rowspan", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("tfoot"))
        .element(
            relaxed_element("th")
                .attribute("abbr", Pattern::any())
                .attribute("axis", Pattern::any())
                .attribute("colspan", Pattern::any())
                .attribute("rowspan", Pattern::any())
                .attribute("scope", Pattern::any())
                .attribute("width", Pattern::any()),
        )
        .element(relaxed_element("thead"))
        .element(
            relaxed_element("time")
                .attribute("datetime", Pattern::any())
                .attribute("pubdate", Pattern::any()),
        )
        .element(relaxed_element("tr"))
        .element(relaxed_element("u"))
        .element(relaxed_element("ul").attribute("type", Pattern::any()))
        .element(relaxed_element("var"))
        .element(relaxed_element("wbr"))
        .space("address")
        .space("article")
        .space("aside")
        .space("footer")
        .space("header")
        .space("hr")
        .space("nav")
        .space("section")
}

fn restricted() -> Rules {
    Rules::new()
        .element(Element::new("b"))
        .element(Element::new("em"))
        .element(Element::new("i"))
        .element(Element::new("strong"))
        .element(Element::new("u"))
        .space("address")
        .space("article")
        .space("aside")
        .space("blockquote")
        .space("br")
        .space("dd")
        .space("div")
        .space("dl")
        .space("dt")
        .space("footer")
        .space("h1")
        .space("h2")
        .space("h3")
        .space("h4")
        .space("h5")
        .space("h6")
        .space("header")
        .space("hgroup")
        .space("hr")
        .space("li")
        .space("nav")
        .space("ol")
        .space("p")
        .space("pre")
        .space("section")
        .space("ul")
}

fn untrusted() -> Rules {
    Rules::new()
        .element(
            Element::new("a")
                .attribute("href", href())
                .mandatory_attribute("target", "_blank")
                .mandatory_attribute("rel", "noreferrer noopener"),
        )
        .element(Element::new("b"))
        .element(Element::new("em"))
        .element(Element::new("i"))
        .element(Element::new("strong"))
        .element(Element::new("u"))
        .space("address")
        .space("article")
        .space("aside")
        .space("blockquote")
        .space("br")
        .space("dd")
        .space("div")
        .space("dl")
        .space("dt")
        .space("footer")
        .space("h1")
        .space("h2")
        .space("h3")
        .space("h4")
        .space("h5")
        .space("h6")
        .space("header")
        .space("hgroup")
        .space("hr")
        .space("li")
        .space("nav")
        .space("ol")
        .space("p")
        .space("pre")
        .space("section")
        .space("ul")
}

#[cfg(test)]
mod tests {
    use super::{basic, default, relaxed, restricted, untrusted};

    #[test]
    fn basic_does_not_fail() {
        let rules = basic();
        assert_eq!(rules.allowed_elements.len(), 31);
    }

    #[test]
    fn default_does_not_fail() {
        let rules = default();
        assert_eq!(rules.allowed_elements.len(), 0);
        assert_eq!(rules.space_elements.len(), 26);
        assert_eq!(rules.delete_elements.len(), 6);
    }

    #[test]
    fn relaxed_does_not_fail() {
        let rules = relaxed();
        assert_eq!(rules.allowed_elements.len(), 58);
        assert_eq!(rules.space_elements.len(), 8);
    }

    #[test]
    fn restricted_does_not_fail() {
        let rules = restricted();
        assert_eq!(rules.allowed_elements.len(), 5);
        assert_eq!(rules.space_elements.len(), 26);
    }

    #[test]
    fn untrusted_does_not_fail() {
        let rules = untrusted();
        assert_eq!(rules.allowed_elements.len(), 6);
        assert_eq!(rules.space_elements.len(), 26);
    }
}
