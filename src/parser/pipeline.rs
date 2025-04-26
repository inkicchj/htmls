use super::Parser;
use crate::lexer::Token;
use crate::parser::ast::Node;
use crate::parser::error::ParseError;
use super::basic::parse_basic;

/// Parsing pipeline operation expressions.
pub fn parse_pipeline(it: &mut Parser) -> Result<Node, ParseError> {

    let mut left = parse_basic(it)?;

    while let Some((Token::Pipeline, _, _)) = &it.current_token {
        it.consume_token(&Token::Pipeline)?;

        it.check_depth()?;

        let right = parse_basic(it)?;

        left = Node::Pipeline(Box::new(left), Box::new(right));

        it.decrease_depth();
    }

    Ok(left)
}
