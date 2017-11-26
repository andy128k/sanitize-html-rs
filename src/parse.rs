use std::error;
use std::default::Default;

type StdResult<T> = Result<T, Box<error::Error>>;

use html5ever::{parse_fragment, serialize};
use html5ever::serialize::{SerializeOpts, TraversalScope};
use html5ever::driver::ParseOpts;
use html5ever::rcdom::{RcDom};
use html5ever::tendril::TendrilSink;
use html5ever::tokenizer::TokenizerOpts;
use html5ever::tree_builder::TreeBuilderOpts;
use html5ever::interface::QualName;

pub fn parse_bytes(input: &[u8]) -> RcDom {
    let opts = ParseOpts {
        tokenizer: TokenizerOpts {
            ..Default::default()
        },
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
    };

    let context_name = QualName::new(Some(namespace_prefix!("html")), ns!(html), local_name!("body"));
    let context_attrs = Vec::new();

    parse_fragment(RcDom::default(), opts, context_name, context_attrs)
        .from_utf8()
        .one(input)
}

pub fn unparse_bytes(dom: RcDom) -> StdResult<Vec<u8>> {
    let mut buf: Vec<u8> = Vec::new();

    let parent = QualName::new(Some(namespace_prefix!("html")), ns!(html), local_name!("div"));

    let opts = SerializeOpts {
        scripting_enabled: false,
        traversal_scope: TraversalScope::ChildrenOnly(Some(parent)),
        create_missing_parent: false,
    };

    serialize(&mut buf, &dom.document, opts)?;

    Ok(buf)
}
