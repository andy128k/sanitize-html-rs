use std::mem;
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use html5ever::rcdom::{RcDom, Handle, Node};
use html5ever::rcdom::NodeData;
use html5ever::tree_builder::TreeSink;
use html5ever::interface::tree_builder::NodeOrText;
use html5ever::interface;
use html5ever::tendril::StrTendril;
use html5ever::{namespace_url, ns};

use super::rules::{Rules, Element};

fn simple_name(name: &str) -> interface::QualName {
    interface::QualName::new(None, ns!(), html5ever::LocalName::from(name))
}

fn name_to_string(name: &interface::QualName) -> String {
    if name.ns == ns!(html) || name.ns.is_empty() {
        name.local.to_lowercase()
    } else {
        format!("{}:{}", name.ns.to_lowercase(), name.local.to_lowercase())
    }
}

fn simple_element(dom: &mut RcDom, name: interface::QualName, attrs: Vec<interface::Attribute>, children: Vec<Handle>) -> Handle {
    let element = dom.create_element(name, attrs, Default::default());
    for child in children {
        dom.remove_from_parent(&child);
        dom.append(&element, NodeOrText::AppendNode(child));
    }
    element
}

fn create_space_text() -> Handle {
    let contents = StrTendril::from(" ");
    Rc::new(Node {
        parent: Cell::new(None),
        children: RefCell::new(Vec::new()),
        data: NodeData::Text { contents: RefCell::new(contents) },
    })
}

enum ElementAction<'t> {
    Keep(&'t Element),
    Delete,
    Space,
    Elide,
    Rename(&'t str),
}

fn element_action<'t>(element_name: &interface::QualName, rules: &'t Rules) -> ElementAction<'t> {
    let name = name_to_string(element_name);
    if name == "html" || name == "body" {
        ElementAction::Elide
    } else if let Some(element_sanitizer) = rules.allowed_elements.get(&name) {
        ElementAction::Keep(element_sanitizer)
    } else if rules.delete_elements.contains(&name) {
        ElementAction::Delete
    } else if rules.space_elements.contains(&name) {
        ElementAction::Space
    } else if let Some(rename_to) = rules.rename_elements.get(&name) {
        ElementAction::Rename(rename_to)
    } else {
        ElementAction::Elide
    }
}

fn clean_nodes(dom: &mut RcDom, nodes: &Vec<Handle>, rules: &Rules) -> Vec<Handle> {
    let mut result = Vec::new();
    for node in nodes.iter() {
        let subnodes = clean_node(dom, node, rules);
        result.extend(subnodes);
    }
    result
}

fn clean_node(dom: &mut RcDom, node: &Handle, rules: &Rules) -> Vec<Handle> {
    match &node.data {
        &NodeData::Document => vec![],
        &NodeData::Doctype { .. } => vec![],
        &NodeData::ProcessingInstruction { .. } => vec![],

        &NodeData::Text { .. } => vec![node.clone()],

        &NodeData::Comment { .. } => if rules.allow_comments { vec![node.clone()] } else { vec![] },

        &NodeData::Element { ref name, ref attrs, .. } => {
            match element_action(name, rules) {
                ElementAction::Keep(element_sanitizer) => {
                    let mut new_attrs: Vec<interface::Attribute> = Vec::new();

                    /* whitelisted attributes */
                    for attr in attrs.borrow().iter() {
                        if element_sanitizer.is_valid(&name_to_string(&attr.name), attr.value.as_ref()) {
                            new_attrs.push(attr.clone());
                        }
                    }

                    /* mandatory attributes */
                    let mut mandatory_attributes: Vec<(&String, &String)> = element_sanitizer.mandatory_attributes.iter().collect();
                    mandatory_attributes.sort();
                    for &(attr_name, attr_value) in mandatory_attributes.iter() {
                        new_attrs.push(interface::Attribute {
                            name: simple_name(&attr_name),
                            value: StrTendril::from(attr_value.as_ref()),
                        });
                    }

                    let children = clean_nodes(dom, &*node.children.borrow(), rules);
                    let element = simple_element(dom, name.clone(), new_attrs, children);

                    vec![element]
                },

                ElementAction::Delete => vec![],

                ElementAction::Elide => clean_nodes(dom, &*node.children.borrow(), rules),

                ElementAction::Space => {
                    let mut nodes = clean_nodes(dom, &*node.children.borrow(), rules);
                    if nodes.is_empty() {
                        nodes.push(create_space_text());
                    } else {
                        nodes.insert(0, create_space_text());
                        nodes.push(create_space_text());
                    }
                    nodes
                },

                ElementAction::Rename(rename_to) => {
                    let children = clean_nodes(dom, &*node.children.borrow(), rules);
                    vec![
                        simple_element(dom, simple_name(rename_to), Vec::new(), children),
                    ]
                },
            }
        },
    }
}

pub fn sanitize_dom(dom: &mut RcDom, mode: &Rules) {
    let children = mem::replace(&mut *dom.document.children.borrow_mut(), Vec::new());

    let new_children = clean_nodes(dom, &children, mode);

    let document = dom.document.clone();

    for child in new_children {
        child.parent.set(None);
        dom.append(&document, NodeOrText::AppendNode(child));
    }
}
