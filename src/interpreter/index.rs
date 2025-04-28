use super::Interpreter;
use super::error::{InterpreterError, InterpreterResult};
use super::result::SelectionResult;
use crate::parser::IndexNode;
use crate::parser::ast::Literal;

pub fn apply_index_selection(it: &mut Interpreter, index: &IndexNode) -> InterpreterResult<()> {
    match index {
        IndexNode::Single(idx) => apply_single_index(it, idx),
        IndexNode::Multiple(indices) => apply_multiple_indices(it, indices),
        IndexNode::Range(start, end, step) => apply_range_indices(it, start, end, step),
    }
}

fn apply_single_index(it: &mut Interpreter, index: &Literal) -> InterpreterResult<()> {
    it.result = match &it.result {
        SelectionResult::Nodes(nodes) => {
            let idx = match index {
                Literal::Int(n) => {
                    let n = *n;
                    let len = nodes.len() as i64;
                    if n >= 0 && n < len {
                        n
                    } else if n < 0 && -(n) <= len {
                        n + len
                    } else {
                        return Err(InterpreterError::IndexOutOfBounds(n as usize, nodes.len()));
                    }
                }
                _ => {
                    return Err(InterpreterError::InvalidArgument(
                        "index selection expects a value of type int.".to_string(),
                    ));
                }
            };

            let selected_node = nodes[idx as usize].clone();
            SelectionResult::with_nodes(vec![selected_node])
        }
        SelectionResult::Texts(texts) => {
            let idx = match index {
                Literal::Int(n) => {
                    let n = *n;
                    let len = texts.len() as i64;
                    if n >= 0 && n < len {
                        n
                    } else if n < 0 && -(n) <= len {
                        n + len
                    } else {
                        return Err(InterpreterError::IndexOutOfBounds(n as usize, texts.len()));
                    }
                }
                _ => {
                    return Err(InterpreterError::InvalidArgument(
                        "index selection expects a value of type int.".to_string(),
                    ));
                }
            };

            let selected_text = texts[idx as usize].clone();
            SelectionResult::with_texts(vec![selected_text])
        }
    };

    Ok(())
}

fn apply_multiple_indices(it: &mut Interpreter, indices: &Vec<Literal>) -> InterpreterResult<()> {
    it.result = match &it.result {
        SelectionResult::Nodes(nodes) => {
            let mut selected_nodes = Vec::with_capacity(indices.len());

            for index in indices {
                let idx = normal_index(index, nodes.len() as i64)?.unwrap();
                selected_nodes.push(nodes[idx as usize].clone());
            }

            SelectionResult::with_nodes(selected_nodes)
        }
        SelectionResult::Texts(texts) => {
            let mut selected_texts = Vec::with_capacity(indices.len());

            for index in indices {
                let idx = normal_index(index, texts.len() as i64)?.unwrap();
                selected_texts.push(texts[idx as usize].clone());
            }

            SelectionResult::with_texts(selected_texts)
        }
    };
    Ok(())
}

fn apply_range_indices(
    it: &mut Interpreter,
    start: &Option<Literal>,
    end: &Option<Literal>,
    step: &Option<Literal>,
) -> InterpreterResult<()> {
    let len = match &it.result {
        SelectionResult::Nodes(nodes) => nodes.len(),
        SelectionResult::Texts(texts) => texts.len(),
    } as i64;

    let start_index = match start {
        Some(v) => normal_index(v, len)?,
        None => None,
    };

    let end_index = match end {
        Some(v) => normal_index(v, len)?,
        None => None,
    };

    let step_value = if let Some(Literal::Int(n)) = step {
        *n
    } else {
        1
    };

    let indices = if step_value.is_positive() {
        let start_index = start_index.unwrap_or(0);
        let end_index = end_index.unwrap_or(len);

        if start_index <= end_index {
            (start_index..end_index)
                .step_by(step_value as usize)
                .map(|item| Literal::Int(item))
                .collect::<Vec<Literal>>()
        } else {
            return Err(InterpreterError::ExecutionError("When the step size is positive, the starting index must be equal or less than the ending index.".to_owned()));
        }
    } else {
        let start_index = start_index.unwrap_or(len - 1);
        let end_index = end_index.unwrap_or(0);

        if start_index >= end_index {
            let mut indexs = Vec::new();
            let mut st = start_index;
            while st >= end_index {
                indexs.push(st);
                st += step_value;
            }

            indexs
                .iter()
                .map(|item| Literal::Int(*item))
                .collect::<Vec<Literal>>()
        } else {
            return Err(InterpreterError::ExecutionError("When the step size is negative, the starting index must be equal or greater than the ending index.".to_owned()));
        }
    };

    apply_multiple_indices(it, &indices)
}

fn normal_index(index: &Literal, len: i64) -> InterpreterResult<Option<i64>> {
    match index {
        Literal::Int(n) => {
            let n = *n;
            if n >= 0 && n < len {
                Ok(Some(n))
            } else if n < 0 && -(n) <= len {
                Ok(Some(n + len))
            } else {
                return Err(InterpreterError::IndexOutOfBounds(n as usize, len as usize));
            }
        }
        _ => {
            return Err(InterpreterError::InvalidArgument(
                "index selection expects a value of type int.".to_string(),
            ));
        }
    }
}
