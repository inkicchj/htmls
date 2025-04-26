use super::Interpreter;
use super::Visitor;
use super::error::{InterpreterError, InterpreterResult};
use crate::parser::ast::Node;

pub fn apply_pipeline(it: &mut Interpreter, left: &Node, right: &Node) -> InterpreterResult<()> {
    it.visit_node(left)?;

    if it.result.is_empty() {
        return Ok(());
    }

    if it.result.is_texts() {
        return Err(InterpreterError::execution_error(
            "The text results on the left side of the pipeline cannot be used as input for the operations on the right side.",
        ));
    }

    it.visit_node(right)?;

    Ok(())
}
