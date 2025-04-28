use super::Token;

use super::{ParseError, Parser, ast::Literal};

pub fn parse_literal(it: &mut Parser) -> Result<Literal, ParseError> {
    let value = match &it.current_token {
        Some((Token::String(v), _, _)) => {
            let v = Literal::Str(v.clone());
            it.read_token();
            v
        }
        Some((Token::Bool(v), _, _)) => {
            let v = Literal::Bool(*v);
            it.read_token();
            v
        }
        Some((Token::Number(v), _, _)) => {
            let v = Literal::Int(*v as i64);
            it.read_token();
            v
        }
        Some((Token::Float(v), _, _)) => {
            let v = Literal::Float(*v);
            it.read_token();
            v
        }
        Some((Token::Minus, _, _)) => {
            it.read_token();

            match &it.current_token {
                Some((Token::Number(v), _, _)) => {
                    let v = Literal::Int(-(*v as i64));
                    it.read_token();
                    v
                }
                Some((Token::Float(v), _, _)) => {
                    let v = Literal::Float(-(*v));
                    it.read_token();
                    v
                }
                _ => {
                    let (line, column) = it.get_current_position();
                    let current = it.get_current_token_str();
                    return Err(ParseError::unexpected_token(
                        "int or float",
                        &current,
                        line,
                        column,
                    ));
                }
            }
        }
        Some((Token::LeftBracket, _, _)) => {
            it.read_token();

            it.check_depth()?;

            let mut list: Vec<Literal> = Vec::new();
            let first_value = parse_literal(it)?;
            list.push(first_value);

            while let Some((Token::Comma, _, _)) = &it.current_token {
                it.read_token();
                let value = parse_literal(it)?;
                list.push(value);
            }

            if let Some((Token::RightBracket, _, _)) = &it.current_token {
                it.read_token();

                it.decrease_depth();

                Literal::List(list)
            } else {
                let (line, column) = it.get_current_position();
                let current = it.get_current_token_str();
                return Err(ParseError::unexpected_token("]", &current, line, column));
            }
        }
        _ => {
            let (line, column) = it.get_current_position();
            let current = it.get_current_token_str();
            return Err(ParseError::unexpected_token(
                "str, int, float, and bool",
                &current,
                line,
                column,
            ));
        }
    };
    Ok(value)
}
