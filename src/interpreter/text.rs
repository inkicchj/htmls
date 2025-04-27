use super::error::{InterpreterError, InterpreterResult};
use super::result::NodeHandle;
use super::{Interpreter, SelectionResult, html};
use crate::parser::ast::TextNode;

/// Apply text selector
pub fn apply_text_selector(it: &mut Interpreter, text_node: &TextNode) -> InterpreterResult<()> {
    let nodes = it.result.nodes()?;

    let result = match text_node {
        TextNode::Text => extract_text_content(&nodes)?,
        TextNode::Href => extract_href_values(&nodes)?,
        TextNode::Src => extract_src_values(&nodes)?,
        TextNode::AttrValue(name, is_regex) => extract_attr_values(&nodes, name, *is_regex)?,
    };

    it.result = SelectionResult::with_texts(result);

    Ok(())
}

/// Extract text content from nodes
fn extract_text_content(nodes: &Vec<NodeHandle>) -> InterpreterResult<Vec<String>> {
    let mut text_values = Vec::with_capacity(nodes.len());

    for node in nodes {
        match html::extract_text(node) {
            Ok(text) => text_values.push(text),
            Err(err) => {
                return Err(InterpreterError::TextExtractionError(format!(
                    "Unable to extract text content: {}",
                    err
                )));
            }
        }
    }

    Ok(text_values)
}

/// Extract href attribute values from nodes
fn extract_href_values(nodes: &Vec<NodeHandle>) -> InterpreterResult<Vec<String>> {
    let mut href_values = Vec::new();

    for node in nodes {
        match html::get_href(node) {
            Ok(Some(href)) => href_values.push(href),
            Ok(None) => {} // Node has no href attribute, skip
            Err(err) => {
                return Err(InterpreterError::AttributeExtractionError(format!(
                    "Unable to extract href attribute: {}",
                    err
                )));
            }
        }
    }

    Ok(href_values)
}

/// Extract src attribute values from nodes
fn extract_src_values(nodes: &Vec<NodeHandle>) -> InterpreterResult<Vec<String>> {
    let mut src_values = Vec::new();

    for node in nodes {
        match html::get_src(node) {
            Ok(Some(src)) => src_values.push(src),
            Ok(None) => {} // Node has no src attribute, skip
            Err(err) => {
                return Err(InterpreterError::AttributeExtractionError(format!(
                    "Unable to extract src attribute: {}",
                    err
                )));
            }
        }
    }

    Ok(src_values)
}


/// Extract attribute values from nodes
fn extract_attr_values(nodes: &Vec<NodeHandle>, name: &str, is_regex: bool) -> InterpreterResult<Vec<String>> {
    let mut src_values = Vec::new();

    for node in nodes {
        match html::get_attribute(node, name, is_regex){
            Ok(Some(src)) => src_values.push(src),
            Ok(None) => {} // Node has no src attribute, skip
            Err(err) => {
                return Err(InterpreterError::AttributeExtractionError(format!(
                    "Unable to extract src attribute: {}",
                    err
                )));
            }
        }
    }

    Ok(src_values)
}
