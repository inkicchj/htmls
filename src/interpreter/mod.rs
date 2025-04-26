pub mod element;
pub mod error;
pub mod function;
pub mod html;
pub mod index;
pub mod pipeline;
pub mod result;
pub mod set;
pub mod text;

use super::{
    parse,
    parser::{
        ElementNode, FunctionNode, IndexNode, Node, SelectorNode, SetOperationNode, TextNode,
        Visitable, Visitor,
    },
};

pub use error::{InterpreterError, InterpreterResult};
pub use result::{NodeHandle, SelectionResult};



/// HTML selector interpreter
#[derive(Clone)]
pub struct Interpreter {
    /// HTML document tree root node
    pub document: NodeHandle,

    /// Current selection result
    pub result: SelectionResult,

    /// Flag indicating if it's the first interpretation
    pub is_first_interpret: bool,
}

impl Interpreter {

    pub fn new(html: &str) -> InterpreterResult<Self> {

        let document = html::parse_html(html)?;

        Ok(Interpreter {
            document: document.clone(),
            result: SelectionResult::with_nodes(vec![document]),
            is_first_interpret: true,
        })
    }

    /// Reset selection state
    fn reset_selection(&mut self) {
        self.result = SelectionResult::with_nodes(vec![self.document.clone()]);
    }


    /// Select matching nodes
    pub fn select(&mut self, selector: &str) -> InterpreterResult<SelectionResult> {
        // Parse selector into AST
        let ast = parse(selector).map_err(|e| InterpreterError::ParserError(e.to_string()))?;

        // No need to reset on first call, already initialized in new()
        // Need to reset selection state for subsequent calls
        if !self.is_first_interpret {
            self.reset_selection();
        } else {
            // Mark as no longer first interpretation
            self.is_first_interpret = false;
        }

        ast.accept(self)?;

        // Create and return a copy of the result
        Ok(self.result.clone())
    }




    /// Select nodes in specified context
    pub fn select_from(
        &mut self,
        context: &SelectionResult,
        selector: &str,
    ) -> InterpreterResult<SelectionResult> {
        // Save original state
        let original_result = self.result.clone();
        let was_first = self.is_first_interpret;

        // Set context as current result
        self.result = context.clone();
        self.is_first_interpret = false; // Ensure selection state is not reset


        let ast = parse(selector).map_err(|e| InterpreterError::ParserError(e.to_string()))?;
        self.visit_node(&ast)?;

        let result = self.result.clone();

        // Restore original state
        self.result = original_result;
        self.is_first_interpret = was_first;

        Ok(result)
    }
}

/// Implement Visitor trait to traverse and execute AST
impl Visitor<InterpreterResult<()>> for Interpreter {
    fn visit_node(&mut self, node: &Node) -> InterpreterResult<()> {
        match node {
            Node::Selector(selector) => self.visit_selector(selector),
            Node::Pipeline(left, right) => self.visit_pipeline(left, right),
            Node::IndexSelection(inner, index) => {
                self.visit_node(inner)?;
                self.visit_index(index)
            }
            Node::SetOperation(op) => {
                self.visit_set_operation(op)
            }
            Node::FunctionCall(inner, func) => {
                self.visit_node(inner)?;
                self.visit_function(func)
            }
        }
    }

    fn visit_selector(&mut self, selector: &SelectorNode) -> InterpreterResult<()> {
        match selector {
            SelectorNode::ElementSelector(elem) => self.visit_element(elem),
            SelectorNode::TextSelector(text) => self.visit_text(text),
        }
    }

    fn visit_element(&mut self, elem: &ElementNode) -> InterpreterResult<()> {
        element::apply_element_selector(self, elem)?;
        Ok(())
    }

    fn visit_text(&mut self, text: &TextNode) -> InterpreterResult<()> {
        text::apply_text_selector(self, text)?;
        Ok(())
    }

    fn visit_set_operation(&mut self, node: &SetOperationNode) -> InterpreterResult<()> {
        set::apply_set_operation(self, node)?;
        Ok(())
    }

    fn visit_index(&mut self, node: &IndexNode) -> InterpreterResult<()> {
        index::apply_index_selection(self, node)?;
        Ok(())
    }

    fn visit_function(&mut self, node: &FunctionNode) -> InterpreterResult<()> {
        function::apply_function(self, node)?;
        Ok(())
    }

    fn visit_pipeline(&mut self, left: &Node, right: &Node) -> InterpreterResult<()> {
        pipeline::apply_pipeline(self, left, right)?;
        Ok(())
    }
}
