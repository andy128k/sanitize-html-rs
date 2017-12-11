#![cfg(test)]

use super::sanitize_str;
use super::rules::{Element, Rules};
use super::rules::predefined::*;

/* basic */

const BASIC_HTML: &str = "<b>Lo<!-- comment -->rem</b> <a href=\"pants\" title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br/>amet <script>alert(\"hello world\");</script>";

#[test]
fn basic_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, BASIC_HTML).unwrap(),
        "Lorem ipsum dolor sit amet alert(\"hello world\");"
    );
}

#[test]
fn basic_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, BASIC_HTML).unwrap(),
        "<b>Lorem</b> ipsum <strong>dolor</strong> sit amet alert(\"hello world\");"
    );
}

#[test]
fn basic_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, BASIC_HTML).unwrap(),
        "<b>Lorem</b> <a href=\"pants\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br>amet alert(\"hello world\");"
    );
}

#[test]
fn basic_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, BASIC_HTML).unwrap(),
        "<b>Lorem</b> <a href=\"pants\" title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br>amet alert(\"hello world\");"
    );
}

/* malformed */

const MALFORMED_HTML: &str = "Lo<!-- comment -->rem</b> <a href=pants title=\"foo>ipsum <a href=\"http://foo.com/\"><strong>dolor</a></strong> sit<br/>amet <script>alert(\"hello world\");";

#[test]
fn malformed_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, MALFORMED_HTML).unwrap(),
        "Lorem dolor sit amet alert(\"hello world\");"
    );
}

#[test]
fn malformed_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, MALFORMED_HTML).unwrap(),
        "Lorem <strong>dolor</strong> sit amet alert(\"hello world\");"
    );
}

#[test]
fn malformed_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, MALFORMED_HTML).unwrap(),
       "Lorem <a href=\"pants\"><strong>dolor</strong></a> sit<br>amet alert(\"hello world\");"
    );
}

#[test]
fn malformed_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, MALFORMED_HTML).unwrap(),
        "Lorem <a href=\"pants\" title=\"foo>ipsum <a href=\"><strong>dolor</strong></a> sit<br>amet alert(\"hello world\");"
    );
}

/* unclosed */

const UNCLOSED_HTML: &str = "<p>a</p><blockquote>b";

#[test]
fn unclosed_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, UNCLOSED_HTML).unwrap(),
        " a  b "
    );
}

#[test]
fn unclosed_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, UNCLOSED_HTML).unwrap(),
        " a  b "
    );
}

#[test]
fn unclosed_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, UNCLOSED_HTML).unwrap(),
        "<p>a</p><blockquote>b</blockquote>"
    );
}

#[test]
fn unclosed_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, UNCLOSED_HTML).unwrap(),
        "<p>a</p><blockquote>b</blockquote>"
    );
}

/* malicious */

const MALICIOUS_HTML: &str = "<b>Lo<!-- comment -->rem</b> <a href=\"javascript:pants\" title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br/>amet <<foo>script>alert(\"hello world\");</script>";

#[test]
fn malicious_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, MALICIOUS_HTML).unwrap(),
        "Lorem ipsum dolor sit amet &lt;script&gt;alert(\"hello world\");"
    );
}

#[test]
fn malicious_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, MALICIOUS_HTML).unwrap(),
        "<b>Lorem</b> ipsum <strong>dolor</strong> sit amet &lt;script&gt;alert(\"hello world\");"
    );
}

#[test]
fn malicious_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, MALICIOUS_HTML).unwrap(),
        "<b>Lorem</b> <a>ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br>amet &lt;script&gt;alert(\"hello world\");"
    );
}

#[test]
fn malicious_untrusted() {
    assert_eq!(
        &sanitize_str(&UNTRUSTED, MALICIOUS_HTML).unwrap(),
        "<b>Lorem</b> <a rel=\"noreferrer noopener\" target=\"_blank\">ipsum</a> <a href=\"http://foo.com/\" rel=\"noreferrer noopener\" target=\"_blank\"><strong>dolor</strong></a> sit amet &lt;script&gt;alert(\"hello world\");"
    );
}

#[test]
fn malicious_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, MALICIOUS_HTML).unwrap(),
        "<b>Lorem</b> <a title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br>amet &lt;script&gt;alert(\"hello world\");"
    );
}

/* raw-comment */

const RAW_COMMENT_HTML: &str = "<!-- comment -->Hello";

#[test]
fn raw_comment_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, RAW_COMMENT_HTML).unwrap(),
        "Hello"
    );
}

#[test]
fn raw_comment_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, RAW_COMMENT_HTML).unwrap(),
        "Hello"
    );
}

#[test]
fn raw_comment_basic() {
    assert_eq!(&sanitize_str(&BASIC, RAW_COMMENT_HTML).unwrap(), "Hello");
}

#[test]
fn raw_comment_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, RAW_COMMENT_HTML).unwrap(),
        "Hello"
    );
}

/* protocol-based JS injection: simple, no spaces */

const JS_INJECTION_HTML_1: &str = "<a href=\"javascript:alert(\'XSS\');\">foo</a>";

#[test]
fn js_injection_1_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_1).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_1_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_1).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_1_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_1).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_1_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_1).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: simple, spaces before */

const JS_INJECTION_HTML_2: &str = "<a href=\"javascript :alert(\'XSS\');\">foo</a>";

#[test]
fn js_injection_2_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_2).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_2_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_2).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_2_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_2).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_2_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_2).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: simple, spaces after */

const JS_INJECTION_HTML_3: &str = "<a href=\"javascript: alert(\'XSS\');\">foo</a>";

#[test]
fn js_injection_3_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_3).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_3_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_3).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_3_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_3).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_3_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_3).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: simple, spaces before and after */

const JS_INJECTION_HTML_4: &str = "<a href=\"javascript : alert(\'XSS\');\">foo</a>";

#[test]
fn js_injection_4_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_4).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_4_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_4).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_4_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_4).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_4_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_4).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: preceding colon */

const JS_INJECTION_HTML_5: &str = "<a href=\":javascript:alert(\'XSS\');\">foo</a>";

#[test]
fn js_injection_5_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_5).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_5_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_5).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_5_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_5).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_5_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_5).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: UTF-8 encoding */

const JS_INJECTION_HTML_6: &str = "<a href=\"javascript&#58;\">foo</a>";

#[test]
fn js_injection_6_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_6).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_6_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_6).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_6_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_6).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_6_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_6).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: long UTF-8 encoding */

const JS_INJECTION_HTML_7: &str = "<a href=\"javascript&#0058;\">foo</a>";

#[test]
fn js_injection_7_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_7).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_7_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_7).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_7_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_7).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_7_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_7).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: long UTF-8 encoding without semicolons */

const JS_INJECTION_HTML_8: &str = "<a href=&#0000106&#0000097&#0000118&#0000097&#0000115&#0000099&#0000114&#0000105&#0000112&#0000116&#0000058&#0000097&#0000108&#0000101&#0000114&#0000116&#0000040&#0000039&#0000088&#0000083&#0000083&#0000039&#0000041>foo</a>";

#[test]
fn js_injection_8_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_8).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_8_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_8).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_8_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_8).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_8_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_8).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: hex encoding */

const JS_INJECTION_HTML_9: &str = "<a href=\"javascript&#x3A;\">foo</a>";

#[test]
fn js_injection_9_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_9).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_9_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_9).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_9_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_9).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_9_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_9).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: long hex encoding */

const JS_INJECTION_HTML_10: &str = "<a href=\"javascript&#x003A;\">foo</a>";

#[test]
fn js_injection_10_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_10).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_10_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_10).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_10_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_10).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_10_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_10).unwrap(),
        "<a>foo</a>"
    );
}

/* protocol-based JS injection: hex encoding without semicolons */

const JS_INJECTION_HTML_11: &str = "<a href=&#x6A&#x61&#x76&#x61&#x73&#x63&#x72&#x69&#x70&#x74&#x3A&#x61&#x6C&#x65&#x72&#x74&#x28&#x27&#x58&#x53&#x53&#x27&#x29>foo</a>";

#[test]
fn js_injection_11_default() {
    assert_eq!(
        &sanitize_str(&DEFAULT, JS_INJECTION_HTML_11).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_11_restricted() {
    assert_eq!(
        &sanitize_str(&RESTRICTED, JS_INJECTION_HTML_11).unwrap(),
        "foo"
    );
}

#[test]
fn js_injection_11_basic() {
    assert_eq!(
        &sanitize_str(&BASIC, JS_INJECTION_HTML_11).unwrap(),
        "<a>foo</a>"
    );
}

#[test]
fn js_injection_11_relaxed() {
    assert_eq!(
        &sanitize_str(&RELAXED, JS_INJECTION_HTML_11).unwrap(),
        "<a>foo</a>"
    );
}

/* should translate valid HTML entities */

#[test]
fn misc_1() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "Don&apos;t tas&eacute; me &amp; bro!").unwrap(),
        "Don't tasé me &amp; bro!"
    );
}

/* should translate valid HTML entities while encoding unencoded ampersands */

#[test]
fn misc_2() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "cookies&sup2; & &frac14; cr&eacute;me").unwrap(),
        "cookies² &amp; ¼ créme"
    );
}

/* should never output &apos; */

#[test]
fn misc_3() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "<a href='&apos;' class=\"' &#39;\">IE6 isn't a real browser</a>").unwrap(),
        "IE6 isn't a real browser"
    );
}

/* should not choke on several instances of the same element in a row */

#[test]
fn misc_4() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "<img src=\"http://www.google.com/intl/en_ALL/images/logo.gif\"><img src=\"http://www.google.com/intl/en_ALL/images/logo.gif\"><img src=\"http://www.google.com/intl/en_ALL/images/logo.gif\"><img src=\"http://www.google.com/intl/en_ALL/images/logo.gif\">").unwrap(),
        ""
    );
}

/* should surround the contents of :whitespace_elements with space characters when removing the element */

#[test]
fn misc_5() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "foo<div>bar</div>baz").unwrap(),
        "foo bar baz"
    );
}

#[test]
fn misc_6() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "foo<br>bar<br>baz").unwrap(),
        "foo bar baz"
    );
}

#[test]
fn misc_7() {
    assert_eq!(
        &sanitize_str(&DEFAULT, "foo<hr>bar<hr>baz").unwrap(),
        "foo bar baz"
    );
}

#[test]
fn custom_rules() {
    let rules = Rules::new()
        .allow_comments(true)
        .element(Element::new("b"))
        .element(Element::new("span"))
        .delete("script")
        .delete("style")
        .space("br")
        .rename("strong", "span")
    ;

    let html = "<b>Lo<!-- comment -->rem</b> <a href=\"javascript:pants\" title=\"foo\">ipsum</a> <a href=\"http://foo.com/\"><strong>dolor</strong></a> sit<br/>amet <script>alert(\"hello world\")</script>";

    assert_eq!(
        &sanitize_str(&rules, html).unwrap(),
        "<b>Lo<!-- comment -->rem</b> ipsum <span>dolor</span> sit amet "
    );
}
