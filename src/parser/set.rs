use crate::lexer::Token;
use crate::parser::ast::{Node, SetOperationNode};
use crate::parser::error::ParseError;
use super::pipeline::parse_pipeline;
use super::Parser;

/// Parsing set operation expressions.
pub fn parse_set(it: &mut Parser) -> Result<Node, ParseError> {
    let mut left = parse_pipeline(it)?;

    loop {
        match &it.current_token {
            Some((Token::Union, _, _)) => {
                it.consume_token(&Token::Union)?;

                it.check_depth()?;
                let right = parse_pipeline(it)?;

                left = Node::SetOperation(Box::new(SetOperationNode::Union(
                    Box::new(left),
                    Box::new(right),
                )));

                it.decrease_depth();
            }

            Some((Token::Intersection, _, _)) => {

                it.consume_token(&Token::Intersection)?;

                it.check_depth()?;

                let right = parse_pipeline(it)?;

                left = Node::SetOperation(Box::new(SetOperationNode::Intersection(
                    Box::new(left),
                    Box::new(right),
                )));

                it.decrease_depth();
            }

            Some((Token::Difference, _, _)) => {

                it.consume_token(&Token::Difference)?;

                it.check_depth()?;

                let right = parse_pipeline(it)?;

                left = Node::SetOperation(Box::new(SetOperationNode::Difference(
                    Box::new(left),
                    Box::new(right),
                )));

                it.decrease_depth();
            }

            _ => break,
        }
    }

    Ok(left)
}
