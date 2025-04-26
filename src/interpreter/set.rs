use std::collections::HashSet;

use crate::Node;
use crate::SetOperationNode;

use super::Visitor;
use super::error::InterpreterError;
use super::{Interpreter, InterpreterResult, SelectionResult};
use super::result::NodeHandle;

/// Apply set operation
pub fn apply_set_operation(it: &mut Interpreter, node: &SetOperationNode) -> InterpreterResult<()> {
    match node {
        SetOperationNode::Union(left, right) => union_operation(it, left, right)?,
        SetOperationNode::Intersection(left, right) => intersection_operation(it, left, right)?,
        SetOperationNode::Difference(left, right) => difference_operation(it, left, right)?,
    }

    Ok(())
}

/// Set operation result type
enum OperationResults {
    Nodes(Vec<NodeHandle>, Vec<NodeHandle>),
    Texts(Vec<String>, Vec<String>),
}

/// Execute both sides of the node and return results
fn execute_sides(
    it: &Interpreter,
    left: &Box<Node>,
    right: &Box<Node>,
    op_name: &str,
) -> InterpreterResult<OperationResults> {
    // Execute left and right nodes, using clone but optimized to only copy necessary parts
    let mut it_left = it.clone();
    let mut it_right = it.clone();

    it_left.visit_node(left)?;
    it_right.visit_node(right)?;

    // Get results from both sides
    let left_results = it_left.result;
    let right_results = it_right.result;

    // Ensure both sides have consistent result types
    if left_results.is_nodes() != right_results.is_nodes() {
        return Err(InterpreterError::execution_error(
            &format!("{} operation has inconsistent result types: left side is {}, right side is {}",
                op_name,
                if left_results.is_nodes() { "nodes" } else { "texts" },
                if right_results.is_nodes() { "nodes" } else { "texts" }
            )
        ));
    }

    // Return different result sets based on the result type
    if left_results.is_nodes() {
        let left_nodes = left_results.nodes()?.clone();
        let right_nodes = right_results.nodes()?.clone();
        Ok(OperationResults::Nodes(left_nodes, right_nodes))
    } else {
        let left_texts = left_results.texts()?.clone();
        let right_texts = right_results.texts()?.clone();
        Ok(OperationResults::Texts(left_texts, right_texts))
    }
}

/// Union operation
fn union_operation(
    it: &mut Interpreter,
    left: &Box<Node>,
    right: &Box<Node>,
) -> InterpreterResult<()> {
    // Execute both sides of the node and get results
    match execute_sides(it, left, right, "union")? {
        OperationResults::Nodes(left_nodes, right_nodes) => {
            // Estimate result size to optimize memory allocation
            let estimated_size = left_nodes.len() + right_nodes.len();
            
            // Use hash set for deduplication
            let mut seen_ids = HashSet::with_capacity(estimated_size);
            let mut result = Vec::with_capacity(estimated_size);

            // Add left side results
            for node in left_nodes {
                let id = node.id().to_string();
                if seen_ids.insert(id) {
                    result.push(node);
                }
            }

            // Add unseen right side results
            for node in right_nodes {
                let id = node.id().to_string();
                if seen_ids.insert(id) {
                    result.push(node);
                }
            }

            it.result = SelectionResult::with_nodes(result);
        },
        OperationResults::Texts(left_texts, right_texts) => {
            // Estimate result size
            let estimated_size = left_texts.len() + right_texts.len();
            
            // Use hash set for deduplication
            let mut seen_texts = HashSet::with_capacity(estimated_size);
            let mut result = Vec::with_capacity(estimated_size);

            // Add left side texts
            for text in left_texts {
                if seen_texts.insert(text.clone()) {
                    result.push(text);
                }
            }

            // Add unseen right side texts
            for text in right_texts {
                if seen_texts.insert(text.clone()) {
                    result.push(text);
                }
            }

            it.result = SelectionResult::with_texts(result);
        }
    }

    Ok(())
}

/// Intersection operation
fn intersection_operation(
    it: &mut Interpreter,
    left: &Box<Node>,
    right: &Box<Node>,
) -> InterpreterResult<()> {
    // Execute both sides of the node and get results
    match execute_sides(it, left, right, "intersection")? {
        OperationResults::Nodes(left_nodes, right_nodes) => {
            // Create hash set of left side node IDs, pre-allocate capacity
            let left_ids: HashSet<String> = left_nodes
                .iter()
                .map(|node| node.id().to_string())
                .collect();

            // Estimate result capacity (worst case is all from right side)
            let mut node_result = Vec::with_capacity(right_nodes.len());
            
            // Filter right side nodes, only keep nodes with IDs appearing on the left side
            for node in right_nodes {
                let node_id = node.id().to_string();
                if left_ids.contains(&node_id) {
                    node_result.push(node);
                }
            }

            it.result = SelectionResult::with_nodes(node_result);
        },
        OperationResults::Texts(left_texts, right_texts) => {
            // Create hash set of left side texts
            let left_text_set: HashSet<String> = left_texts.into_iter().collect();
            
            // Estimate result capacity
            let mut text_result = Vec::with_capacity(right_texts.len());
            
            // Filter right side texts, only keep texts appearing on the left side
            for text in right_texts {
                if left_text_set.contains(&text) {
                    text_result.push(text);
                }
            }

            it.result = SelectionResult::with_texts(text_result);
        }
    }

    Ok(())
}

/// Difference operation
fn difference_operation(
    it: &mut Interpreter,
    left: &Box<Node>,
    right: &Box<Node>,
) -> InterpreterResult<()> {
    // Execute both sides of the node and get results
    match execute_sides(it, left, right, "difference")? {
        OperationResults::Nodes(left_nodes, right_nodes) => {
            // Create hash set of right side node IDs, pre-allocate capacity
            let right_ids: HashSet<String> = right_nodes
                .iter()
                .map(|node| node.id().to_string())
                .collect();

            // Estimate result capacity (worst case is all from left side)
            let mut node_result = Vec::with_capacity(left_nodes.len());
            
            // Filter left side nodes, exclude nodes with IDs appearing on the right side
            for node in left_nodes {
                let node_id = node.id().to_string();
                if !right_ids.contains(&node_id) {
                    node_result.push(node);
                }
            }

            it.result = SelectionResult::with_nodes(node_result);
        },
        OperationResults::Texts(left_texts, right_texts) => {
            // Create hash set of right side texts
            let right_text_set: HashSet<String> = right_texts.into_iter().collect();
            
            // Estimate result capacity
            let mut text_result = Vec::with_capacity(left_texts.len());
            
            // Filter left side texts, exclude texts appearing on the right side
            for text in left_texts {
                if !right_text_set.contains(&text) {
                    text_result.push(text);
                }
            }

            it.result = SelectionResult::with_texts(text_result);
        }
    }

    Ok(())
}
 