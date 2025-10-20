use crate::rcdom::{Node, NodeData, RcDom};
use crate::rules::{Element, Rules};
use html5ever::{Attribute, LocalName, interface::QualName, ns, tendril::StrTendril};
use std::{cell::RefCell, rc::Rc};

fn simple_qual_name(name: &str) -> QualName {
    QualName::new(None, ns!(), LocalName::from(name))
}

fn qual_name_to_string(name: &QualName) -> String {
    if name.ns == ns!(html) || name.ns.is_empty() {
        name.local.to_lowercase()
    } else {
        format!("{}:{}", name.ns.to_lowercase(), name.local.to_lowercase())
    }
}

fn simple_element(name: QualName, attrs: Vec<Attribute>, children: Vec<Rc<Node>>) -> Rc<Node> {
    let element = Node::new(NodeData::Element {
        name,
        attrs: RefCell::new(attrs),
        template_contents: Default::default(),
        mathml_annotation_xml_integration_point: Default::default(),
    });
    element.children.borrow_mut().extend(children);
    element
}

fn create_space_text() -> Rc<Node> {
    Node::new(NodeData::Text {
        contents: RefCell::new(" ".into()),
    })
}

enum ElementAction<'t> {
    Keep(&'t Element),
    Delete,
    Space,
    Elide,
    Rename(&'t str),
}

fn element_action<'t>(element_name: &QualName, rules: &'t Rules) -> ElementAction<'t> {
    let name = qual_name_to_string(element_name);
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

fn clean_nodes(nodes: &[Rc<Node>], rules: &Rules) -> Vec<Rc<Node>> {
    nodes
        .iter()
        .flat_map(|node| clean_node(node, rules))
        .collect()
}

fn clean_node(node: &Rc<Node>, rules: &Rules) -> Vec<Rc<Node>> {
    match node.data {
        NodeData::Document => vec![],
        NodeData::Doctype { .. } => vec![],
        NodeData::ProcessingInstruction { .. } => vec![],

        NodeData::Text { .. } => vec![node.clone()],

        NodeData::Comment { .. } => {
            if rules.allow_comments {
                vec![node.clone()]
            } else {
                vec![]
            }
        }

        NodeData::Element {
            ref name,
            ref attrs,
            ..
        } => {
            match element_action(name, rules) {
                ElementAction::Keep(element_sanitizer) => {
                    let mut new_attrs: Vec<Attribute> = Vec::new();

                    /* allowlisted attributes */
                    for attr in attrs.borrow().iter() {
                        if element_sanitizer.is_valid(&qual_name_to_string(&attr.name), &attr.value)
                        {
                            new_attrs.push(attr.clone());
                        }
                    }

                    /* mandatory attributes */
                    let mut mandatory_attributes: Vec<(&String, &String)> =
                        element_sanitizer.mandatory_attributes.iter().collect();
                    mandatory_attributes.sort();
                    for &(attr_name, attr_value) in mandatory_attributes.iter() {
                        new_attrs.push(Attribute {
                            name: QualName {
                                prefix: None,
                                ns: ns!(),
                                local: LocalName::from(attr_name.as_str()),
                            },
                            value: StrTendril::from(attr_value.as_str()),
                        });
                    }

                    let children = clean_nodes(&node.children.borrow(), rules);
                    let element = simple_element(name.clone(), new_attrs, children);

                    vec![element]
                }

                ElementAction::Delete => vec![],

                ElementAction::Elide => clean_nodes(&node.children.borrow(), rules),

                ElementAction::Space => {
                    let mut nodes = clean_nodes(&node.children.borrow(), rules);
                    if nodes.is_empty() {
                        nodes.push(create_space_text());
                    } else {
                        nodes.insert(0, create_space_text());
                        nodes.push(create_space_text());
                    }
                    nodes
                }

                ElementAction::Rename(rename_to) => {
                    let children = clean_nodes(&node.children.borrow(), rules);
                    vec![simple_element(
                        simple_qual_name(rename_to),
                        Vec::new(),
                        children,
                    )]
                }
            }
        }
    }
}

pub(crate) fn sanitize_dom(dom: &RcDom, mode: &Rules) -> Rc<Node> {
    let new_children = clean_nodes(&dom.document.children.borrow(), mode);

    let new_dom = Node::new(NodeData::Document);
    new_dom.children.borrow_mut().extend(new_children);
    new_dom
}
