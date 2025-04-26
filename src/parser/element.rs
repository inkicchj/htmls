use crate::lexer::Token;
use crate::parser::ast::{ElementNode, SelectorNode};
use crate::parser::error::ParseError;
use super::Parser;

/// Parse element selector
pub fn parse_element(it: &mut Parser) -> Result<SelectorNode, ParseError> {
    match &it.current_token {
        Some((Token::Class, _, _)) => {
            let element_node = parse_class_selector(it)?;
            Ok(SelectorNode::ElementSelector(element_node))
        }
        Some((Token::Id, _, _)) => {
            let element_node = parse_id_selector(it)?;
            Ok(SelectorNode::ElementSelector(element_node))
        }
        Some((Token::Tag, _, _)) => {
            let element_node = parse_tag_selector(it)?;
            Ok(SelectorNode::ElementSelector(element_node))
        }
        Some((Token::Attr, _, _)) => {
            let element_node = parse_attr_selector(it)?;
            Ok(SelectorNode::ElementSelector(element_node))
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            Err(ParseError::unexpected_token(
                "class, id, tag, or attr",
                &current,
                line,
                column,
            ))
        }
    }
}

/// Parse class selector (class)
fn parse_class_selector(it: &mut Parser) -> Result<ElementNode, ParseError> {
    it.consume_token(&Token::Class)?;
    
    let (is_regex, value) = parse_selector_value(it)?;

    if value.is_empty() {
        let (line, column) = it.get_current_position();
        return Err(ParseError::invalid_selector_value("empty value", line, column));
    }

    Ok(ElementNode::Class(value, is_regex))
}

/// Parse ID selector (id)
fn parse_id_selector(it: &mut Parser) -> Result<ElementNode, ParseError> {

    it.consume_token(&Token::Id)?;

    let (is_regex, value) = parse_selector_value(it)?;

    if value.is_empty() {
        let (line, column) = it.get_current_position();
        return Err(ParseError::invalid_selector_value("empty value", line, column));
    }

    Ok(ElementNode::Id(value, is_regex))
}

/// Parse tag selector (tag)
fn parse_tag_selector(it: &mut Parser) -> Result<ElementNode, ParseError> {

    it.consume_token(&Token::Tag)?;

    let (is_regex, value) = parse_selector_value(it)?;

    if value.is_empty() {
        let (line, column) = it.get_current_position();
        return Err(ParseError::invalid_selector_value("empty value", line, column));
    }

    Ok(ElementNode::Tag(value, is_regex))
}

/// Parse attribute selector (attr)
fn parse_attr_selector(it: &mut Parser) -> Result<ElementNode, ParseError> {

    it.consume_token(&Token::Attr)?;
   
    let (_, attr_name) = parse_selector_value(it)?;

    if attr_name.is_empty() {
        let (line, column) = it.get_current_position();
        return Err(ParseError::invalid_selector_value("empty attribute name", line, column));
    }

    let (is_regex, attr_value) = match parse_selector_value(it) {
        Ok((is_regex, attr_value)) => (is_regex, Some(attr_value)),
        Err(_) => (false, None)
    };

    Ok(ElementNode::Attr(attr_name, attr_value, is_regex))
}

/// Parse selector value
fn parse_selector_value(it: &mut Parser) -> Result<(bool, String), ParseError> {
    let is_regex = if it.check_token(&Token::Regex) {
        it.read_token(); // Consume regex token
        true
    } else {
        false
    };

    // Parse selector value (common argument or quoted argument)
    match &it.current_token {
        Some((Token::Argument(value), _, _)) => {
            let value = value.clone();
            it.read_token(); // Consume argument
            Ok((is_regex, value))
        }
        Some((Token::QuotedArgument(value), _, _)) => {
            let value = value.clone();
            it.read_token(); // Consume argument
            Ok((is_regex, value))
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            Err(ParseError::unexpected_token(
                "selector argument",
                &current,
                line,
                column,
            ))
        }
    }
}
