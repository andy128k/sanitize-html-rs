use super::errors::SanitizeError;
use html5ever::{
    interface::QualName,
    local_name, namespace_prefix, namespace_url, ns, serialize,
    serialize::{SerializeOpts, TraversalScope},
    tendril::TendrilSink,
};
use kuchiki::{parse_html_with_options, NodeRef, ParseOpts};
use std::default::Default;

pub(crate) fn parse_str(input: &str) -> NodeRef {
    let mut opts = ParseOpts::default();
    opts.tree_builder.drop_doctype = true;

    let mut parser = parse_html_with_options(opts);
    parser.process(input.into());
    parser.finish()
}

pub(crate) fn unparse_bytes(dom: &NodeRef) -> Result<Vec<u8>, SanitizeError> {
    let mut buf: Vec<u8> = Vec::new();

    let parent = QualName::new(Some(namespace_prefix!("html")), ns!(html), local_name!("div"));

    let opts = SerializeOpts {
        scripting_enabled: false,
        traversal_scope: TraversalScope::ChildrenOnly(Some(parent)),
        create_missing_parent: false,
    };

    serialize(&mut buf, dom, opts).map_err(SanitizeError::SerializeError)?;

    Ok(buf)
}
