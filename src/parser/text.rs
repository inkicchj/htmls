use crate::lexer::Token;
use crate::parser::ast::{SelectorNode, TextNode};
use crate::parser::error::ParseError;
use super::Parser;

/// Parse Text Selector
pub fn parse_text(it: &mut Parser) -> Result<SelectorNode, ParseError> {
    match &it.current_token {
        Some((Token::Text, _, _)) => {
            let text_node = parse_plain_text_selector(it)?;
            Ok(SelectorNode::TextSelector(text_node))
        }
        Some((Token::Href, _, _)) => {
            let text_node = parse_href_selector(it)?;
            Ok(SelectorNode::TextSelector(text_node))
        }
        Some((Token::Src, _, _)) => {
            let text_node = parse_src_selector(it)?;
            Ok(SelectorNode::TextSelector(text_node))
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            Err(ParseError::unexpected_token(
                "text,href,src",
                &current,
                line,
                column,
            ))
        }
    }
}

/// Parse the plain text selector (text)
fn parse_plain_text_selector(it: &mut Parser) -> Result<TextNode, ParseError> {

    it.consume_token(&Token::Text)?;

    Ok(TextNode::Text)
}

/// Parsing the href attribute selector (href)
fn parse_href_selector(it: &mut Parser) -> Result<TextNode, ParseError> {
    it.consume_token(&Token::Href)?;

    Ok(TextNode::Href)
}

/// Parsing the src attribute selector (src)
fn parse_src_selector(it: &mut Parser) -> Result<TextNode, ParseError> {
    it.consume_token(&Token::Src)?;

    Ok(TextNode::Src)
}
