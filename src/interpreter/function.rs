use crate::{FunctionNode, parser::ast::Literal};

use super::{Interpreter, InterpreterError, InterpreterResult};

pub fn apply_function(it: &mut Interpreter, node: &FunctionNode) -> InterpreterResult<()> {
    let texts = it.result.texts_mut()?;

    match node.name.as_str() {
        "trim" => trim(texts),
        "replace" => {
            if node.arguments.len() != 2 {
                return Err(InterpreterError::MissingArgument(
                    "repalce must include 2 argument.".to_string(),
                ));
            };
            let value0 = match &node.arguments[0] {
                Literal::Str(v) => v,
                _ => {
                    return Err(InterpreterError::InvalidArgument(
                        "The first parameter of replace expects a value of type str.".to_string(),
                    ));
                }
            };
            let value1 = match &node.arguments[1] {
                Literal::Str(v) => v,
                _ => {
                    return Err(InterpreterError::InvalidArgument(
                        "The 2th parameter of replace expects a value of type str.".to_string(),
                    ));
                }
            };
            replace(texts, value0, value1)
        }
        "lowercase" => lowercase(texts),
        "uppercase" => uppercase(texts),
        "join" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::Str(v) => v,
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "join expects a value of type str".to_string(),
                        ));
                    }
                }
            } else if node.arguments.len() == 0 {
                ""
            } else {
                return Err(InterpreterError::MissingArgument(
                    "join must include 0 or 1 argument.".to_string(),
                ));
            };
            join(texts, value0)
        }
        "format" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::Str(v) => v,
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "format expect a value of type str".to_string(),
                        ));
                    }
                }
            } else {
                return Err(InterpreterError::MissingArgument(
                    "format must include 1 argument.".to_string(),
                ));
            };
            format(texts, value0)
        }
        "contains" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::Str(v) => v,
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "contains expect a value of type str".to_string(),
                        ));
                    }
                }
            } else {
                return Err(InterpreterError::MissingArgument(
                    "contains must include 1 argument.".to_string(),
                ));
            };
            contains(texts, value0);
        }
        "starts_with" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::Str(v) => v,
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "starts_with expect a value of type str".to_string(),
                        ));
                    }
                }
            } else {
                return Err(InterpreterError::MissingArgument(
                    "starts_with must include 1 argument.".to_string(),
                ));
            };
            starts_with(texts, value0);
        }
        "ends_with" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::Str(v) => v,
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "ends_with expect a value of type str".to_string(),
                        ));
                    }
                }
            } else {
                return Err(InterpreterError::MissingArgument(
                    "ends_with must include 1 argument.".to_string(),
                ));
            };
            ends_with(texts, value0);
        }
        "in" => {
            let value0 = if node.arguments.len() == 1 {
                match &node.arguments[0] {
                    Literal::List(list) => {
                        let mut values = Vec::new();
                        for item in list {
                            match item {
                                Literal::Str(v) => values.push(v.clone()),
                                _ => {
                                    return Err(InterpreterError::InvalidArgument(
                                        "in expect a value of type list<str>".to_string(),
                                    ));
                                }
                            };
                        }
                        values
                    }
                    _ => {
                        return Err(InterpreterError::InvalidArgument(
                            "in expect a value of type list<str>".to_string(),
                        ));
                    }
                }
            } else {
                return Err(InterpreterError::MissingArgument(
                    "in must include 1 argument.".to_string(),
                ));
            };

            in_(texts, value0);
        }
        _ => return Err(InterpreterError::UnknownFunction(node.name.clone())),
    };

    Ok(())
}

fn trim(texts: &mut Vec<String>) {
    texts
        .iter_mut()
        .for_each(|text| *text = text.trim().to_string());
}

fn replace(texts: &mut Vec<String>, search: &str, replacement: &str) {
    texts
        .iter_mut()
        .for_each(|text| *text = text.replace(search, replacement))
}

fn lowercase(texts: &mut Vec<String>) {
    texts
        .iter_mut()
        .for_each(|text| *text = text.to_lowercase())
}

fn uppercase(texts: &mut Vec<String>) {
    texts
        .iter_mut()
        .for_each(|text| *text = text.to_uppercase())
}

fn join(texts: &mut Vec<String>, separator: &str) {
    *texts = vec![texts.join(separator)]
}

fn format(texts: &mut Vec<String>, format_str: &str) {
    texts.iter_mut().for_each(|text| {
        if format_str.contains("{}") {
            *text = format!("{}", format_str.replacen("{}", &text, 1));
        } else if format_str.starts_with("%") {
            match format_str {
                "%s" => { /* Keep as is, equivalent to {} */ }
                "%d" | "%i" => {
                    if let Ok(num) = text.trim().parse::<i64>() {
                        *text = num.to_string();
                    }
                }
                "%f" => {
                    if let Ok(num) = text.trim().parse::<f64>() {
                        *text = num.to_string();
                    }
                }
                "%x" => {
                    if let Ok(num) = text.trim().parse::<i64>() {
                        *text = format!("{:x}", num);
                    }
                }
                "%X" => {
                    if let Ok(num) = text.trim().parse::<i64>() {
                        *text = format!("{:X}", num);
                    }
                }
                _ => {
                    *text = format!("{}{}", format_str, text);
                }
            }
        } else {
            *text = format!("{}{}", format_str, text);
        }
    })
}

fn contains(texts: &mut Vec<String>, inner: &str) {
    let mut result = Vec::new();

    for text in texts.iter() {
        if text.contains(inner) {
            result.push(text.clone());
        }
    }

    *texts = result;
}

fn starts_with(texts: &mut Vec<String>, st: &str) {
    let mut result = Vec::new();

    for text in texts.iter() {
        if text.starts_with(st) {
            result.push(text.clone());
        }
    }

    *texts = result;
}

fn ends_with(texts: &mut Vec<String>, ed: &str) {
    let mut result = Vec::new();

    for text in texts.iter() {
        if text.ends_with(ed) {
            result.push(text.clone());
        }
    }

    *texts = result;
}

fn in_(texts: &mut Vec<String>, list: Vec<String>) {
    let mut result = Vec::new();
    for text in texts.iter() {
        if list.contains(text) {
            result.push(text.clone());
        }
    }

    *texts = result;
}
