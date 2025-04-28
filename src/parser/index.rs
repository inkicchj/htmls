use super::{Parser, literal};
use crate::lexer::Token;
use crate::parser::ast::{IndexNode, Node};
use crate::parser::error::ParseError;

/// 解析索引选择
pub fn parse_index(it: &mut Parser, mut node: Node) -> Result<Node, ParseError> {
    if let Some((Token::Colon, _, _)) = &it.current_token {
        node = parse_index_selector(it, node)?;
    }

    Ok(node)
}

/// 解析索引选择器
pub fn parse_index_selector(it: &mut Parser, node: Node) -> Result<Node, ParseError> {
    if !it.check_token(&Token::Colon) {
        let (line, column) = it.get_current_position();
        let current = it.get_current_token_str();
        return Err(ParseError::unexpected_token(":", &current, line, column));
    }

    it.consume_token(&Token::Colon)?;

    it.check_depth()?;

    let index_node = match &it.current_token {
        Some((Token::Colon, _, _)) => {
            it.read_token();
            let end_index = if let Some((Token::Colon, _, _)) = &it.current_token {
                it.read_token();
                None
            } else {
                Some(literal::parse_literal(it)?)
            };

            let step_value = if let Some((Token::Colon, _, _)) = &it.current_token {
                it.read_token();
                Some(literal::parse_literal(it)?)
            } else {
                match literal::parse_literal(it) {
                    Ok(step) => Some(step),
                    Err(_) => None,
                }
            };

            IndexNode::Range(None, end_index, step_value)
        }
        _ => {
            let start_index = literal::parse_literal(it)?;
            match &it.current_token {
                Some((Token::Colon, _, _)) => {
                    it.read_token();
                    let end_index = if let Some((Token::Colon, _, _)) = &it.current_token {
                        it.read_token();
                        None
                    } else {
                        Some(literal::parse_literal(it)?)
                    };

                    let step_value = if let Some((Token::Colon, _, _)) = &it.current_token {
                        it.read_token();
                        Some(literal::parse_literal(it)?)
                    } else {
                        match literal::parse_literal(it) {
                            Ok(step) => Some(step),
                            Err(_) => None,
                        }
                    };

                    IndexNode::Range(Some(start_index), end_index, step_value)
                }
                Some((Token::Comma, _, _)) => {
                    it.read_token();

                    let mut indexs = vec![start_index];

                    while let Some((Token::Comma, _, _)) = &it.current_token {
                        it.read_token();
                        let index = literal::parse_literal(it)?;
                        indexs.push(index);
                    }

                    IndexNode::Multiple(indexs)
                }
                _ => IndexNode::Single(start_index),
            }
        }
    };

    it.decrease_depth();

    Ok(Node::IndexSelection(Box::new(node), Box::new(index_node)))
}
