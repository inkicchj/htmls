use crate::Token;

use super::{Node, ParseError, Parser};
use super::element::parse_element;
use super::set::parse_set;
use super::text::parse_text;
use super::index::parse_index;
use super::function::parse_function;


/// Parse basic selector expression
pub fn parse_basic(it: &mut Parser) -> Result<Node, ParseError> {
    match &it.current_token {
        Some((Token::Class, _, _))
        | Some((Token::Id, _, _))
        | Some((Token::Tag, _, _))
        | Some((Token::Attr, _, _)) => {

            let selector = parse_element(it)?;
            let node = Node::Selector(Box::new(selector));

            parse_index(it, node)
        }
        Some((Token::Text, _, _)) | Some((Token::Href, _, _)) | Some((Token::Src, _, _)) | Some((Token::Pound, _, _)) => {
            let selector = parse_text(it)?;
            let node = Node::Selector(Box::new(selector));

            let node = parse_index(it, node)?;
            parse_function(it, node)
        }
        Some((Token::LeftParen, _, _)) => {
            it.consume_token(&Token::LeftParen)?;

            it.check_depth()?;

            let expr = parse_set(it)?;
            match &it.current_token {
                Some((Token::RightParen, _, _)) => {
                    it.consume_token(&Token::RightParen)?;

                    it.decrease_depth();

                    Ok(expr)
                }
                _ => {
                    let (line, column) = it.get_current_position();
                    let current = it.get_current_token_str();
                    Err(ParseError::unexpected_token(
                        "right parenthesis ')'",
                        &current,
                        line,
                        column,
                    ))
                }
            }
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            Err(ParseError::unexpected_token(
                "selector",
                &current,
                line,
                column,
            ))
        }
    }
}