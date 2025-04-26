use crate::lexer::Token;
use crate::parser::ast::{FunctionNode, Node};
use crate::parser::error::ParseError;

use super::Parser;

/// Parse function calls
pub fn parse_function(it: &mut Parser, mut node: Node) -> Result<Node, ParseError> {
    while let Some((Token::Function(function_name), _, _)) = &it.current_token {
        let function_name = function_name.clone();
        if function_name.is_empty() {
            let (line, column) = it.get_current_position();
            return Err(ParseError::syntax_error("Function name cannot be empty.", line, column));
        }
        it.read_token();

        it.check_depth()?;

        let arguments = if let Some((Token::Comma, _, _)) = &it.current_token {
            it.consume_token(&Token::Comma)?;

            parse_function_arguments(it)?
        } else {
            Vec::new()
        };

        let function_node = FunctionNode {
            name: function_name,
            arguments,
        };

        node = Node::FunctionCall(Box::new(node), function_node);

        it.decrease_depth();
    }

    Ok(node)
}

/// Parse the function parameter list
fn parse_function_arguments(it: &mut Parser) -> Result<Vec<String>, ParseError> {
    let mut arguments = Vec::new();

    match &it.current_token {
        Some((Token::QuotedArgument(value), _, _) | (Token::Argument(value), _, _)) => {
            arguments.push(value.clone());
            it.read_token();
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            return Err(ParseError::unexpected_token(
                "Function parameters",
                &current,
                line,
                column,
            ));
        }
    }

    while let Some((Token::Comma, _, _)) = &it.current_token {

        it.consume_token(&Token::Comma)?;

        match &it.current_token {
            Some((Token::QuotedArgument(value), _, _) | (Token::Argument(value), _, _)) => {
                arguments.push(value.clone());
                it.read_token();
            }
            _ => {
                let (line, column) = it.get_current_position();
                let current = it.get_current_token_str();
                return Err(ParseError::unexpected_token(
                    "function parameters",
                    &current,
                    line,
                    column,
                ));
            }
        }
    }

    Ok(arguments)
}
