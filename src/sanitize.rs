use html5ever::{interface::QualName, LocalName, namespace_url, ns};
use kuchiki::{NodeRef, NodeData, ElementData, ExpandedName, Attribute};
use crate::rules::{Rules, Element};

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

fn expanded_name_to_string(name: &ExpandedName) -> String {
    if name.ns == ns!(html) || name.ns.is_empty() {
        name.local.to_lowercase()
    } else {
        format!("{}:{}", name.ns.to_lowercase(), name.local.to_lowercase())
    }
}

fn simple_element(
    name: QualName,
    attrs: Vec<(ExpandedName, Attribute)>,
    children: Vec<NodeRef>,
) -> NodeRef {
    let element = NodeRef::new_element(name, attrs);
    for child in children {
        child.detach();
        element.append(child);
    }
    element
}

fn create_space_text() -> NodeRef {
    NodeRef::new_text(" ")
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

fn clean_nodes(nodes: impl IntoIterator<Item = NodeRef>, rules: &Rules) -> Vec<NodeRef> {
    let mut result = Vec::new();
    for node in nodes {
        let subnodes = clean_node(&node, rules);
        result.extend(subnodes);
    }
    result
}

fn clean_node(node: &NodeRef, rules: &Rules) -> Vec<NodeRef> {
    match node.data() {
        NodeData::Document(..) => vec![],
        NodeData::DocumentFragment => vec![], // TODO: ??
        NodeData::Doctype(..) => vec![],
        NodeData::ProcessingInstruction(..) => vec![],

        NodeData::Text(..) => vec![node.clone()],

        NodeData::Comment(..) => if rules.allow_comments { vec![node.clone()] } else { vec![] },

        NodeData::Element(ElementData { ref name, ref attributes, .. }) => {
            match element_action(name, rules) {
                ElementAction::Keep(element_sanitizer) => {
                    let mut new_attrs: Vec<(ExpandedName, Attribute)> = Vec::new();

                    /* whitelisted attributes */
                    for (attr_name, attr_value) in attributes.borrow().map.iter() {
                        if element_sanitizer.is_valid(&expanded_name_to_string(attr_name), &attr_value.value) {
                            new_attrs.push((attr_name.clone(), attr_value.clone()));
                        }
                    }

                    /* mandatory attributes */
                    let mut mandatory_attributes: Vec<(&String, &String)> = element_sanitizer.mandatory_attributes.iter().collect();
                    mandatory_attributes.sort();
                    for &(attr_name, attr_value) in mandatory_attributes.iter() {
                        new_attrs.push((
                            ExpandedName::new(ns!(), LocalName::from(attr_name.as_str())),
                            Attribute {
                                prefix: None,
                                value: attr_value.into(),
                            },
                        ));
                    }

                    let children = clean_nodes(node.children(), rules);
                    let element = simple_element(name.clone(), new_attrs, children);

                    vec![element]
                },

                ElementAction::Delete => vec![],

                ElementAction::Elide => clean_nodes(node.children(), rules),

                ElementAction::Space => {
                    let mut nodes = clean_nodes(node.children(), rules);
                    if nodes.is_empty() {
                        nodes.push(create_space_text());
                    } else {
                        nodes.insert(0, create_space_text());
                        nodes.push(create_space_text());
                    }
                    nodes
                },

                ElementAction::Rename(rename_to) => {
                    let children = clean_nodes(node.children(), rules);
                    vec![
                        simple_element(simple_qual_name(rename_to), Vec::new(), children),
                    ]
                },
            }
        },
    }
}

pub(crate) fn sanitize_dom(dom: &NodeRef, mode: &Rules) -> NodeRef {
    let new_children = clean_nodes(dom.children(), mode);
    let new_dom = NodeRef::new_document();
    for child in new_children {
        child.detach();
        new_dom.append(child);
    }
    new_dom
}
