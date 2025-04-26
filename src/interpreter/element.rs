// interpreter/element.rs - Element selector execution module
//
// This module is responsible for executing element selectors, including class, id, tag and attr selectors.
// It relies on the DOM manipulation functionality provided by html.rs to implement precise selection of HTML elements.

use super::error::InterpreterResult;
use super::result::{NodeHandle, SelectionResult};
use super::{Interpreter, html};
use crate::parser::ElementNode;
use regex;

/// Apply element selector
pub fn apply_element_selector(
    it: &mut Interpreter,
    elem_node: &ElementNode,
) -> InterpreterResult<()> {
    // Ensure current result is of node type
    let nodes = it.result.nodes()?;

    // Select nodes based on element selector type
    let result = match elem_node {
        ElementNode::Class(class_name, is_regex) => select_by_class(nodes, class_name, *is_regex)?,
        ElementNode::Id(id, is_regex) => select_by_id(nodes, id, *is_regex)?,
        ElementNode::Tag(tag_name, is_regex) => select_by_tag(nodes, tag_name, *is_regex)?,
        ElementNode::Attr(attr_name, attr_value, is_regex) => {
            let value_ref = attr_value.as_ref().map(|s| s.as_str());
            select_by_attr(nodes, attr_name, value_ref, *is_regex)?
        },
    };

    // Wrap result as SelectionResult
    it.result = SelectionResult::with_nodes(result);

    Ok(())
}

/// Select elements by class attribute
fn select_by_class(
    current_selection: &Vec<NodeHandle>,
    class_name: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let mut result = Vec::new();

    // Apply class selector to each currently selected node
    for node in current_selection {
        let mut matches = html::find_by_class(node, class_name, is_regex)?;
        result.append(&mut matches);
    }

    Ok(result)
}

/// Select elements by id attribute
fn select_by_id(
    current_selection: &Vec<NodeHandle>,
    id: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let mut result = Vec::new();

    // Apply id selector to each currently selected node
    for node in current_selection {
        let mut matches = html::find_by_id(node, id, is_regex)?;
        result.append(&mut matches);
    }

    Ok(result)
}

/// Select elements by tag name
fn select_by_tag(
    current_selection: &Vec<NodeHandle>,
    tag_name: &str,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let mut result = Vec::new();

    // Apply tag selector to each currently selected node
    for node in current_selection {
        let mut matches = html::find_by_tag(node, tag_name, is_regex)?;
        result.append(&mut matches);
    }

    Ok(result)
}

/// Select elements by attribute name
fn select_by_attr(
    current_selection: &Vec<NodeHandle>,
    attr_name: &str,
    attr_value: Option<&str>,
    is_regex: bool,
) -> InterpreterResult<Vec<NodeHandle>> {
    let mut result = Vec::new();

    // Handle special case: attr value "name"
    if attr_name.is_empty() && attr_value.is_some() {
        // Search all attributes by attribute value
        for node in current_selection {
            // Get all attributes
            let attributes = html::get_all_attributes(node)?;
            
            let target_value = attr_value.unwrap();
            
            // Check if any attribute value matches the target value
            let mut found = false;
            for (_, val) in attributes {
                let is_match = if is_regex {
                    // Regex matching
                    match regex::Regex::new(target_value) {
                        Ok(re) => re.is_match(&val),
                        Err(_) => false,
                    }
                } else {
                    // Exact matching
                    val == target_value
                };
                
                if is_match {
                    found = true;
                    break;
                }
            }
            
            if found {
                result.push(node.clone());
            }
            
            // Recursively process child nodes
            let children = html::get_children(node)?;
            for child in children {
                let mut child_results = select_by_attr(&vec![child], attr_name, attr_value, is_regex)?;
                result.append(&mut child_results);
            }
        }
    } else {
        // Regular case: search by attribute name and optional attribute value
        for node in current_selection {
            let mut matches = html::find_by_attr(node, attr_name, attr_value, is_regex)?;
            result.append(&mut matches);
        }
    }

    Ok(result)
}