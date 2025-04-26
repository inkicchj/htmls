use crate::FunctionNode;

use super::{Interpreter, InterpreterError, InterpreterResult};

pub fn apply_function(it: &mut Interpreter, node: &FunctionNode) -> InterpreterResult<()> {
    let texts = it.result.texts_mut()?;

    match node.name.as_str() {
        "trim" => trim(texts),
        "replace" => {
            if node.arguments.len() < 2 {
                return Err(InterpreterError::MissingArgument(
                    "replace requires two arguments: search pattern and replacement text".to_string(),
                ));
            }

            let search = &node.arguments[0];
            let replacement = &node.arguments[1];
            replace(texts, search, replacement)
        }
        "lowercase" => lowercase(texts),
        "uppercase" => uppercase(texts),
        "join" => {
            let separator = node.arguments.get(0).map(|s| s.as_str()).unwrap_or("");
            join(texts, separator)
        }
        "format" => {
            if node.arguments.len() != 1 {
                return Err(InterpreterError::MissingArgument(
                    "format requires one argument: format string".to_string(),
                ));
            }

            let format_str = &node.arguments[0];
            format(texts, format_str)
        },
        "contains" => {
            if node.arguments.len() != 1 {
                return Err(InterpreterError::MissingArgument(
                    "contains requires one argument".to_string(),
                ));
            }

            contains(texts, &node.arguments[0]);
        }
        "starts_with" => {
            if node.arguments.len() != 1 {
                return Err(InterpreterError::MissingArgument(
                    "starts_with requires one argument".to_string(),
                ));
            }

            starts_with(texts, &node.arguments[0]);
        }
        "ends_with" => {
            if node.arguments.len() != 1 {
                return Err(InterpreterError::MissingArgument(
                    "ends_with requires one argument".to_string(),
                ));
            }

            ends_with(texts, &node.arguments[0]);
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
    texts
        .iter_mut()
        .for_each(|text| {
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
    };

    *texts = result;

}


fn starts_with(texts: &mut Vec<String>, st: &str) {
    
    let mut result = Vec::new();

    for text in texts.iter() {
        if text.starts_with(st) {
            result.push(text.clone());
        }
    };

    *texts = result;

}

fn ends_with(texts: &mut Vec<String>, ed: &str) {
    
    let mut result = Vec::new();

    for text in texts.iter() {
        if text.ends_with(ed) {
            result.push(text.clone());
        }
    };

    *texts = result;

}