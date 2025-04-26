use super::Interpreter;
use super::error::{InterpreterError, InterpreterResult};
use super::result::SelectionResult;
use crate::parser::IndexNode;


pub fn apply_index_selection(it: &mut Interpreter, index: &IndexNode) -> InterpreterResult<()> {
    match index {
        IndexNode::Single(idx) => apply_single_index(it, *idx),
        IndexNode::Multiple(indices) => apply_multiple_indices(it, indices),
        IndexNode::Range(start, end, step) => apply_range_indices(it, *start, *end, *step),
    }
}

fn apply_single_index(it: &mut Interpreter, index: usize) -> InterpreterResult<()> {
    it.result = match &it.result {
        SelectionResult::Nodes(nodes) => {
            if index >= nodes.len() {
                return Err(InterpreterError::IndexOutOfBounds(index, nodes.len()));
            }

            let selected_node = nodes[index].clone();
            SelectionResult::with_nodes(vec![selected_node])
        }
        SelectionResult::Texts(texts) => {
            if index >= texts.len() {
                return Err(InterpreterError::IndexOutOfBounds(index, texts.len()));
            }

            let selected_text = texts[index].clone();
            SelectionResult::with_texts(vec![selected_text])
        }
    };

    Ok(())
}

fn apply_multiple_indices(it: &mut Interpreter, indices: &[usize]) -> InterpreterResult<()> {
    it.result = match &it.result {
        SelectionResult::Nodes(nodes) => {
            let mut selected_nodes = Vec::with_capacity(indices.len());

            for &idx in indices {
                if idx >= nodes.len() {
                    return Err(InterpreterError::IndexOutOfBounds(idx, nodes.len()));
                }
                selected_nodes.push(nodes[idx].clone());
            }

            SelectionResult::with_nodes(selected_nodes)
        }
        SelectionResult::Texts(texts) => {
            let mut selected_texts = Vec::with_capacity(indices.len());

            for &idx in indices {
                if idx >= texts.len() {
                    return Err(InterpreterError::IndexOutOfBounds(idx, texts.len()));
                }
                selected_texts.push(texts[idx].clone());
            }

            SelectionResult::with_texts(selected_texts)
        }
    };
    Ok(())
}

fn apply_range_indices(
    it: &mut Interpreter,
    start: usize,
    end: usize,
    step: Option<usize>,
) -> InterpreterResult<()> {
    let step_value = step.unwrap_or(1);
    if step_value == 0 {
        return Err(InterpreterError::InvalidStep(step_value));
    }
    let indices: Vec<usize> = (start..=end).step_by(step_value).collect();

    // Reuse the logic of multi-index selection
    apply_multiple_indices(it, &indices)
}
