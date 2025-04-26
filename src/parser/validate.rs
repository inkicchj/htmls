use crate::parser::ast::{
    ElementNode, FunctionNode, IndexNode, Node, SelectorNode, SetOperationNode,
    TextNode, Visitor, Visitable,
};
use crate::parser::error::ParseError;

/// Syntax validator state
#[derive(Debug, Clone)]
struct ValidationState {
    /// Whether a text query directive has already appeared in the current path
    has_text_selector: bool,
    
    /// Position where the text query directive appeared (line, column)
    text_selector_pos: Option<(usize, usize)>,
    
    /// Current position (line, column)
    current_pos: (usize, usize),
}

impl ValidationState {
    /// Create new validation state
    fn new() -> Self {
        ValidationState {
            has_text_selector: false,
            text_selector_pos: None,
            current_pos: (0, 0),
        }
    }
    
    /// Update current position
    fn set_position(&mut self, line: usize, column: usize) {
        self.current_pos = (line, column);
    }
    
    /// Record text selector appearance
    fn mark_text_selector(&mut self) {
        self.has_text_selector = true;
        self.text_selector_pos = Some(self.current_pos);
    }
    
    /// Check if element selector can be added
    fn can_add_element_selector(&self) -> bool {
        !self.has_text_selector
    }
    
}

/// HTML selector syntax validator
pub struct SyntaxValidator {
    /// Current validation state
    state: ValidationState,
}

impl SyntaxValidator {

    pub fn new() -> Self {
        SyntaxValidator {
            state: ValidationState::new(),
        }
    }
    
    /// Validate if AST conforms to syntax rules
    pub fn validate(&mut self, node: &Node) -> Result<(), ParseError> {
        // Use Visitable's accept method instead of directly calling visit_node
        node.accept(self)
    }
    
    /// Set current position information
    #[allow(dead_code)]
    pub fn set_position(&mut self, line: usize, column: usize) {
        self.state.set_position(line, column);
    }
}

impl Visitor<Result<(), ParseError>> for SyntaxValidator {
    fn visit_node(&mut self, node: &Node) -> Result<(), ParseError> {
        match node {
            Node::Selector(selector) => self.visit_selector(selector),
            Node::Pipeline(left, right) => self.visit_pipeline(left, right),
            Node::SetOperation(op) => self.visit_set_operation(op),
            Node::IndexSelection(inner, idx) => {
                self.visit_node(inner)?;
                self.visit_index(idx)
            }
            Node::FunctionCall(inner, func) => {
                self.visit_node(inner)?;
                self.visit_function(func)
            }
        }
    }
    
    fn visit_selector(&mut self, node: &SelectorNode) -> Result<(), ParseError> {
        match node {
            SelectorNode::ElementSelector(elem) => self.visit_element(elem),
            SelectorNode::TextSelector(text) => self.visit_text(text),
        }
    }
    
    fn visit_element(&mut self, _node: &ElementNode) -> Result<(), ParseError> {
        // Check if element selector can be added
        if !self.state.can_add_element_selector() {
            let (line, column) = self.state.current_pos;
            return Err(ParseError::element_after_text_selector(line, column));
        }
        
        Ok(())
    }
    
    fn visit_text(&mut self, _node: &TextNode) -> Result<(), ParseError> {
        // Check if text selector already exists
        if self.state.has_text_selector {
            let (line, column) = self.state.current_pos;
            return Err(ParseError::multiple_text_selectors(line, column));
        }
        
        // Mark text selector appearance
        self.state.mark_text_selector();
        
        Ok(())
    }
    
    fn visit_pipeline(&mut self, left: &Node, right: &Node) -> Result<(), ParseError> {
        // Pipeline operation does not reset text selector state
        // Left operand's state is passed to right operand
        self.visit_node(left)?;
        self.visit_node(right)
    }
    
    
    fn visit_set_operation(&mut self, node: &SetOperationNode) -> Result<(), ParseError> {
        match node {
            SetOperationNode::Union(left, right) 
            | SetOperationNode::Intersection(left, right) 
            | SetOperationNode::Difference(left, right) => {

                let original_state = self.state.clone();
                
                self.visit_node(left)?;
                
                self.state = original_state;
                self.visit_node(right)?;
                
                // State is not merged after set operation, equivalent to reset
                // Because each branch produces independent result sets
                self.state = ValidationState::new();
                
                Ok(())
            }
        }
    }
    
    fn visit_index(&mut self, _node: &IndexNode) -> Result<(), ParseError> {
        // Index selection does not change text selector state
        Ok(())
    }
    
    fn visit_function(&mut self, _node: &FunctionNode) -> Result<(), ParseError> {
        // Function call does not change text selector state
        Ok(())
    }
}