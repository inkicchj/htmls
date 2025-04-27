// interpreter/html.rs - HTML parsing and DOM manipulation helper module
//
// This module is responsible for HTML document parsing and DOM tree manipulation,
// providing a series of helper functions to simplify the use of html5ever.

use super::error::{InterpreterError, InterpreterResult};
use super::result::NodeHandle;
use html5ever::driver::ParseOpts;
use html5ever::parse_document;
use html5ever::tendril::TendrilSink;
use html5ever::tree_builder::TreeBuilderOpts;
use markup5ever_rcdom::{NodeData, RcDom};
use std::collections::HashMap;
use std::default::Default;

/// Parse HTML document and return document root node
pub fn parse_html(html: &str) -> InterpreterResult<NodeHandle> {
    let opts = ParseOpts {
        tree_builder: TreeBuilderOpts {
            drop_doctype: true,
            ..Default::default()
        },
        ..Default::default()
    };

    let dom = parse_document(RcDom::default(), opts)
        .from_utf8()
        .read_from(&mut html.as_bytes())
        .map_err(|e| InterpreterError::html_parse_error(&format!("HTML parsing error: {:?}", e)))?;

    let document = dom.document;

    Ok(NodeHandle::from_html5(document))
}

/// Get child nodes from node handle
pub fn get_children(node: &NodeHandle) -> InterpreterResult<Vec<NodeHandle>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    // Get child nodes
    let children = handle.children.borrow();
    let mut result = Vec::with_capacity(children.len());

    // Wrap each child node as NodeHandle
    for child in children.iter() {
        result.push(NodeHandle::from_html5(child.clone()));
    }

    Ok(result)
}

/// Extract text content from node
pub fn extract_text(node: &NodeHandle) -> InterpreterResult<String> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    // If it's a text node, return text content directly
    if let NodeData::Text { contents } = &handle.data {
        let text = contents.borrow();
        return Ok(text.to_string());
    }

    // If it's an element node, recursively extract text content from all child nodes
    if node.is_element() {
        let mut result = String::new();
        let children = get_children(node)?;

        for child in children {
            result.push_str(&extract_text(&child)?);
        }

        return Ok(result);
    }

    // Other types of nodes have no text content
    Ok(String::new())
}

/// Find elements by tag name
pub fn find_by_tag(
    node: &NodeHandle,
    tag_name: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    let mut result = Vec::new();

    // If current node is an element node, check its tag name
    if let NodeData::Element { name, .. } = &handle.data {
        let current_tag = name.local.to_string();

        let is_match = if is_regex {
            // Use regex for matching
            let re = regex::Regex::new(tag_name).map_err(|e| {
                InterpreterError::execution_error(&format!("Invalid regex pattern: {}", e))
            })?;
            re.is_match(&current_tag)
        } else {
            current_tag == tag_name
        };

        if is_match {
            result.push(node.clone());
        }
    }

    // Recursively search child nodes
    let children = get_children(node)?;
    for child in children {
        let mut child_results = find_by_tag(&child, tag_name, is_regex)?;
        result.append(&mut child_results);
    }

    Ok(result)
}

/// Find elements by class name
pub fn find_by_class(
    node: &NodeHandle,
    class_name: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    let mut result = Vec::new();

    // If current node is an element node, check its class attribute
    if let NodeData::Element { attrs, .. } = &handle.data {
        let attributes = attrs.borrow();
        for attr in attributes.iter() {
            if attr.name.local.to_string() == "class" {
                let class_value = attr.value.to_string();

                // Split class attribute value
                let classes: Vec<&str> = class_value.split_whitespace().collect();

                let is_match = if is_regex {
                    // Use regex to match any class name
                    let re = regex::Regex::new(class_name).map_err(|e| {
                        InterpreterError::execution_error(&format!("Invalid regex pattern: {}", e))
                    })?;
                    classes.iter().any(|c| re.is_match(c))
                } else {
                    // Check if it contains specified class name
                    classes.contains(&class_name)
                };

                if is_match {
                    result.push(node.clone());
                    break;
                }
            }
        }
    }

    // Recursively search child nodes
    let children = get_children(node)?;
    for child in children {
        let mut child_results = find_by_class(&child, class_name, is_regex)?;
        result.append(&mut child_results);
    }

    Ok(result)
}

/// Find elements by ID
pub fn find_by_id(
    node: &NodeHandle,
    id_value: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    let mut result = Vec::new();

    // If current node is an element node, check its id attribute
    if let NodeData::Element { attrs, .. } = &handle.data {
        let attributes = attrs.borrow();
        for attr in attributes.iter() {
            if attr.name.local.to_string() == "id" {
                let current_id = attr.value.to_string();

                let is_match = if is_regex {
                    // Use regex for matching
                    let re = regex::Regex::new(id_value).map_err(|e| {
                        InterpreterError::execution_error(&format!("Invalid regex pattern: {}", e))
                    })?;
                    re.is_match(&current_id)
                } else {
                    current_id == id_value
                };

                if is_match {
                    result.push(node.clone());
                    break;
                }
            }
        }
    }

    // Recursively search child nodes
    let children = get_children(node)?;
    for child in children {
        let mut child_results = find_by_id(&child, id_value, is_regex)?;
        result.append(&mut child_results);
    }

    Ok(result)
}

/// Find elements by attribute
pub fn find_by_attr(
    node: &NodeHandle,
    attr_name: &str,
    attr_value: Option<&str>,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    let mut result = Vec::new();

    // If current node is an element node, check its attributes
    if let NodeData::Element { attrs, .. } = &handle.data {
        let attributes = attrs.borrow();

        for attr in attributes.iter() {
            let current_name = attr.name.local.to_string();

            let name_match = if is_regex {
                // Use regex to match attribute name
                match regex::Regex::new(attr_name) {
                    Ok(re) => re.is_match(&current_name),
                    Err(e) => {
                        return Err(InterpreterError::execution_error(&format!(
                            "Invalid regex pattern: {}",
                            e
                        )));
                    }
                }
            } else {
                current_name == attr_name
            };

            // If attribute value is provided, also need to match
            let value_match = if let Some(value) = attr_value {
                let current_value = attr.value.to_string();

                if is_regex {
                    // Use regex to match attribute value
                    match regex::Regex::new(value) {
                        Ok(re) => re.is_match(&current_value),
                        Err(e) => {
                            return Err(InterpreterError::execution_error(&format!(
                                "Invalid regex pattern: {}",
                                e
                            )));
                        }
                    }
                } else {
                    // Directly compare attribute values
                    current_value == value
                }
            } else {
                // If no attribute value provided, just match attribute name
                true
            };

            if name_match && value_match {
                result.push(node.clone());
                break;
            }
        }
    }

    // Recursively search child nodes
    let children = get_children(node)?;
    for child in children {
        let mut child_results = find_by_attr(&child, attr_name, attr_value, is_regex)?;
        result.append(&mut child_results);
    }

    Ok(result)
}

/// Get element attribute value
pub fn get_attribute(
    node: &NodeHandle,
    attr_name: &str,
    is_regex: bool,
) -> InterpreterResult<Option<String>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    // If not an element node, return None
    if !node.is_element() {
        return Ok(None);
    }

    if let NodeData::Element { attrs, .. } = &handle.data {
        let attributes = attrs.borrow();

        for attr in attributes.iter() {
            let current_name = attr.name.local.to_string();

            let name_match = if is_regex {
                match regex::Regex::new(attr_name) {
                    Ok(re) => re.is_match(&current_name),
                    Err(e) => {
                        return Err(InterpreterError::execution_error(&format!(
                            "Invalid regex pattern: {}",
                            e
                        )));
                    }
                }
            } else {
                current_name == attr_name
            };
            if name_match {
                return Ok(Some(attr.value.to_string()));
            }
        }
    }

    Ok(None)
}

/// Get href attribute value
pub fn get_href(node: &NodeHandle) -> InterpreterResult<Option<String>> {
    get_attribute(node, "href", false)
}

/// Get src attribute value
pub fn get_src(node: &NodeHandle) -> InterpreterResult<Option<String>> {
    get_attribute(node, "src", false)
}

/// Get all node attributes
pub fn get_all_attributes(node: &NodeHandle) -> InterpreterResult<HashMap<String, String>> {
    let handle = node.handle().ok_or_else(|| {
        InterpreterError::execution_error("Node does not have a valid HTML reference")
    })?;

    let mut result = HashMap::new();

    if let NodeData::Element { attrs, .. } = &handle.data {
        let attributes = attrs.borrow();

        for attr in attributes.iter() {
            let name = attr.name.local.to_string();
            let value = attr.value.to_string();
            result.insert(name, value);
        }
    }

    Ok(result)
}
