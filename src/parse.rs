use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::{
    interface::QualName,
    local_name, namespace_prefix, namespace_url, ns, serialize,
    serialize::{SerializeOpts, TraversalScope},
    tendril::TendrilSink,
};
use markup5ever_rcdom::{Node, RcDom, SerializableHandle};
use std::default::Default;
use std::error::Error;
use std::io::Cursor;
use std::rc::Rc;

pub(crate) fn parse_dom(input: &[u8]) -> Result<RcDom, Box<dyn Error>> {
    let mut opts = ParseOpts::default();
    opts.tree_builder.drop_doctype = true;

    let mut cursor = Cursor::new(input);

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut cursor)?;

    Ok(dom)
}

pub(crate) fn unparse_document(document: &Rc<Node>) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut buf: Vec<u8> = Vec::new();

    let parent = QualName::new(
        Some(namespace_prefix!("html")),
        ns!(html),
        local_name!("div"),
    );

    let opts = SerializeOpts {
        scripting_enabled: false,
        traversal_scope: TraversalScope::ChildrenOnly(Some(parent)),
        create_missing_parent: false,
    };

    let document: SerializableHandle = document.clone().into();
    serialize(&mut buf, &document, opts)?;

    Ok(buf)
}
