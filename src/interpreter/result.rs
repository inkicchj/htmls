use super::error::{InterpreterError, InterpreterResult};
use markup5ever_rcdom::{Handle as Html5Handle, NodeData};
use std::fmt;
use std::ptr;


/// HTML node handle type
/// Wraps html5ever's node reference, providing additional functionality
#[derive(Debug, Clone)]
pub struct NodeHandle {
    // HTML node reference
    node: Option<Html5Handle>,

    // Node identifier, used for debugging and comparison
    node_id: String,
}

impl NodeHandle {

    /// Create node from html5ever handle
    pub(crate) fn from_html5(handle: Html5Handle) -> Self {
        // Use the memory address of the underlying node as part of the identifier
        let ptr_value = ptr::addr_of!(*handle) as usize;

        NodeHandle {
            node: Some(handle),
            node_id: format!("node-{:#x}", ptr_value),
        }
    }

    /// Get node ID
    pub fn id(&self) -> &str {
        &self.node_id
    }

    /// Get underlying html5ever handle
    pub fn handle(&self) -> Option<&Html5Handle> {
        self.node.as_ref()
    }

    /// Get node type name
    pub fn node_type(&self) -> &str {
        if let Some(handle) = &self.node {
            match &handle.data {
                NodeData::Document => "Document",
                NodeData::Element { .. } => "Element",
                NodeData::Text { .. } => "Text",
                NodeData::Comment { .. } => "Comment",
                NodeData::ProcessingInstruction { .. } => "ProcessingInstruction",
                NodeData::Doctype { .. } => "Doctype",
            }
        } else {
            "None"
        }
    }

    /// Check if it's an element node
    pub fn is_element(&self) -> bool {
        if let Some(handle) = &self.node {
            matches!(handle.data, NodeData::Element { .. })
        } else {
            false
        }
    }

    /// Check if it's a text node
    pub fn is_text(&self) -> bool {
        if let Some(handle) = &self.node {
            matches!(handle.data, NodeData::Text { .. })
        } else {
            false
        }
    }
}

impl fmt::Display for NodeHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node({})", self.node_id)
    }
}

impl PartialEq for NodeHandle {
    fn eq(&self, other: &Self) -> bool {
        // If both nodes have underlying node references, compare if they point to the same node
        if let (Some(self_handle), Some(other_handle)) = (&self.node, &other.node) {
            ptr::addr_of!(**self_handle) == ptr::addr_of!(**other_handle)
        } else {
            // If there's no underlying node reference, compare IDs
            self.node_id == other.node_id
        }
    }
}

/// Selection result enum
#[derive(Debug, Clone)]
pub enum SelectionResult {
    /// Selected HTML nodes
    Nodes(Vec<NodeHandle>),

    /// Extracted text values
    Texts(Vec<String>),
}

impl SelectionResult {
    /// Create a new empty node result
    pub fn new() -> Self {
        SelectionResult::Nodes(Vec::new())
    }

    /// Create a result with nodes
    pub fn with_nodes(nodes: Vec<NodeHandle>) -> Self {
        SelectionResult::Nodes(nodes)
    }

    /// Create a result with texts
    pub fn with_texts(texts: Vec<String>) -> Self {
        SelectionResult::Texts(texts)
    }

    /// Check if it's a node result
    pub fn is_nodes(&self) -> bool {
        matches!(self, SelectionResult::Nodes(_))
    }

    /// Check if it's a text result
    pub fn is_texts(&self) -> bool {
        matches!(self, SelectionResult::Texts(_))
    }

    /// Get node result, return error if not a node result
    pub fn nodes(&self) -> InterpreterResult<&Vec<NodeHandle>> {
        match self {
            SelectionResult::Nodes(nodes) => Ok(nodes),
            _ => Err(InterpreterError::execution_error("Result type is not nodes")),
        }
    }

    /// Get mutable node result, return error if not a node result
    pub fn nodes_mut(&mut self) -> InterpreterResult<&mut Vec<NodeHandle>> {
        match self {
            SelectionResult::Nodes(nodes) => Ok(nodes),
            _ => Err(InterpreterError::execution_error("Result type is not nodes")),
        }
    }

    /// Get text result, return error if not a text result
    pub fn texts(&self) -> InterpreterResult<&Vec<String>> {
        match self {
            SelectionResult::Texts(texts) => Ok(texts),
            _ => Err(InterpreterError::execution_error("Result type is not texts")),
        }
    }

    /// Get mutable text result, return error if not a text result
    pub fn texts_mut(&mut self) -> InterpreterResult<&mut Vec<String>> {
        match self {
            SelectionResult::Texts(texts) => Ok(texts),
            _ => Err(InterpreterError::execution_error("Result type is not texts")),
        }
    }

    /// Get the number of elements in the result (nodes or texts)
    pub fn count(&self) -> usize {
        match self {
            SelectionResult::Nodes(nodes) => nodes.len(),
            SelectionResult::Texts(texts) => texts.len(),
        }
    }

    /// Check if the result is empty
    pub fn is_empty(&self) -> bool {
        match self {
            SelectionResult::Nodes(nodes) => nodes.is_empty(),
            SelectionResult::Texts(texts) => texts.is_empty(),
        }
    }

    /// Get the first node, return error if not a node result or empty
    pub fn first_node(&self) -> InterpreterResult<&NodeHandle> {
        match self {
            SelectionResult::Nodes(nodes) => nodes
                .first()
                .ok_or_else(|| InterpreterError::execution_error("Node result is empty")),
            _ => Err(InterpreterError::execution_error("Result type is not nodes")),
        }
    }

    /// Get the first text value, return error if not a text result or empty
    pub fn first_text(&self) -> InterpreterResult<&str> {
        match self {
            SelectionResult::Texts(texts) => texts
                .first()
                .ok_or_else(|| InterpreterError::execution_error("Text result is empty"))
                .map(|s| s.as_str()),
            _ => Err(InterpreterError::execution_error("Result type is not texts")),
        }
    }

    /// Get node at specific index, return error if not a node result or index out of bounds
    pub fn node_at(&self, index: usize) -> InterpreterResult<&NodeHandle> {
        match self {
            SelectionResult::Nodes(nodes) => nodes
                .get(index)
                .ok_or_else(|| InterpreterError::IndexOutOfBounds(index, nodes.len())),
            _ => Err(InterpreterError::execution_error("Result type is not nodes")),
        }
    }

    /// Get text value at specific index, return error if not a text result or index out of bounds
    pub fn text_at(&self, index: usize) -> InterpreterResult<&str> {
        match self {
            SelectionResult::Texts(texts) => texts
                .get(index)
                .ok_or_else(|| InterpreterError::IndexOutOfBounds(index, texts.len()))
                .map(|s| s.as_str()),
            _ => Err(InterpreterError::execution_error("Result type is not texts")),
        }
    }

    /// Convert selection result to formatted string
    pub fn to_string_result(&self) -> String {
        match self {
            SelectionResult::Texts(texts) => texts.join("\n"),
            SelectionResult::Nodes(nodes) => {
                if nodes.is_empty() {
                    "Empty result".to_string()
                } else {
                    let node_ids: Vec<String> = nodes.iter().map(|n| n.id().to_string()).collect();
                    format!("Nodes: [{}]", node_ids.join(", "))
                }
            }
        }
    }

    /// Return result iterator
    /// 
    /// This method allows iterating over each element in the result, with each element wrapped as a separate SelectionResult
    pub fn iter(&self) -> SelectionResultIter {
        SelectionResultIter {
            inner: self.clone(),
            index: 0,
        }
    }
}

impl fmt::Display for SelectionResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_result())
    }
}

/// Selection result iterator
#[derive(Debug, Clone)]
pub struct SelectionResultIter {
    inner: SelectionResult,
    index: usize,
}

impl Iterator for SelectionResultIter {
    type Item = SelectionResult;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.inner {
            SelectionResult::Nodes(nodes) => {
                if self.index < nodes.len() {
                    let node = nodes[self.index].clone();
                    self.index += 1;
                    Some(SelectionResult::Nodes(vec![node]))
                } else {
                    None
                }
            },
            SelectionResult::Texts(texts) => {
                if self.index < texts.len() {
                    let text = texts[self.index].clone();
                    self.index += 1;
                    Some(SelectionResult::Texts(vec![text]))
                } else {
                    None
                }
            }
        }
    }
}
 