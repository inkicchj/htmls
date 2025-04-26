use crate::lexer::Token;
use crate::parser::ast::{IndexNode, Node};
use crate::parser::error::ParseError;
use super::Parser;

/// 解析索引选择
pub fn parse_index(it: &mut Parser, mut node: Node) -> Result<Node, ParseError> {
    if let Some((Token::Colon, _, _)) = &it.current_token {
        node = parse_index_selector(it, node)?;
    }

    Ok(node)
}

/// 解析索引选择器
pub fn parse_index_selector(it: &mut Parser, node: Node) -> Result<Node, ParseError> {

    if !matches!(&it.current_token, Some((Token::Colon, _, _))) {
        let (line, column) = it.get_current_position();
        let current = it.get_current_token_str();
        return Err(ParseError::unexpected_token(":", &current, line, column));
    }

    it.consume_token(&Token::Colon)?;

    it.check_depth()?;

    let first_index = parse_single_index(it)?;

    let index_node = match &it.current_token {
        // If it is a colon, it is a range index (:m:n[:s]).
        Some((Token::Colon, _, _)) => {
            it.consume_token(&Token::Colon)?;
            let end_index = parse_single_index(it)?;

            let step = if let Some((Token::Colon, _, _)) = &it.current_token {
                it.consume_token(&Token::Colon)?;

                let step = parse_single_index(it)?;
                Some(step)
            } else {
                None
            };

            IndexNode::Range(first_index, end_index, step)
        }

        // If it is a comma, then it represents multiple indices (:m,n,p).
        Some((Token::Comma, _, _)) => {
            let mut indices = vec![first_index];
           
            while let Some((Token::Comma, _, _)) = &it.current_token {

                it.consume_token(&Token::Comma)?;
                
                let next_index = parse_single_index(it)?;
                indices.push(next_index);
            }

            IndexNode::Multiple(indices)
        }
        _ => IndexNode::Single(first_index),
    };


    it.decrease_depth();

    Ok(Node::IndexSelection(Box::new(node), Box::new(index_node)))
}

/// Parse a single index value
fn parse_single_index(it: &mut Parser) -> Result<usize, ParseError> {
    match &it.current_token {
        Some((Token::Number(number), _, _)) => {
            let value = *number;
            it.read_token();
            Ok(value)
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            Err(ParseError::unexpected_token(
                "number",
                &current,
                line,
                column,
            ))
        }
    }
}
