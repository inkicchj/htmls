use crate::{lexer::Token, tokenize};

pub mod ast;
pub mod element;
pub mod error;
pub mod function;
pub mod index;
pub mod pipeline;
pub mod set;
pub mod text;
pub mod validate;
pub mod basic;
pub mod literal;

use validate::SyntaxValidator;

pub use ast::{
    ElementNode, FunctionNode, IndexNode, Node, SelectorNode, SetOperationNode, TextNode,
    Visitable, Visitor,
};
pub use error::ParseError;

/// HTML selector parser
pub struct Parser {
    /// Token stream: contains tokens, line numbers, and column numbers
    tokens: Vec<(Token, usize, usize)>,

    /// Current position
    position: usize,

    /// Current token
    current_token: Option<(Token, usize, usize)>,

    /// Current nesting depth
    current_depth: usize,

    /// Maximum nesting depth
    max_nesting_level: usize,
}

impl Parser {

    pub fn new(tokens: Vec<(Token, usize, usize)>) -> Self {
        let mut parser = Parser {
            tokens,
            position: 0,
            current_token: None,
            max_nesting_level: 100,
            current_depth: 0,
        };
        parser.read_token();
        parser
    }

    /// Read the next token
    fn read_token(&mut self) {
        if self.position < self.tokens.len() {
            self.current_token = Some(self.tokens[self.position].clone());
            self.position += 1;
        } else {
            self.current_token = None;
        }
    }

    /// Check if the current token matches the expected type
    fn check_token(&self, expected: &Token) -> bool {
        match &self.current_token {
            Some((token, _, _)) => token == expected,
            None => false,
        }
    }

    /// Consume the current token if it matches the expected type
    fn consume_token(&mut self, expected: &Token) -> Result<(), ParseError> {
        if self.check_token(expected) {
            self.read_token();
            Ok(())
        } else {
            let (line, column) = self.get_current_position();
            let current = self.get_current_token_str();
            Err(ParseError::unexpected_token(
                &format!("{}", expected),
                &current,
                line,
                column,
            ))
        }
    }

    /// Get the string representation of the current token
    fn get_current_token_str(&self) -> String {
        match &self.current_token {
            Some((token, _, _)) => format!("{}", token),
            None => "EOF".to_string(),
        }
    }

    /// Get the current position (line number and column number)
    fn get_current_position(&self) -> (usize, usize) {
        match &self.current_token {
            Some((_, line, column)) => (*line, *column),
            None => {
                // If at the end, return the position of the last token
                if !self.tokens.is_empty() {
                    let last = &self.tokens[self.tokens.len() - 1];
                    (last.1, last.2)
                } else {
                    (1, 0) // Default to line 1, column 0
                }
            }
        }
    }

    /// Check nesting depth and increment counter
    fn check_depth(&mut self) -> Result<(), ParseError> {
        if self.current_depth >= self.max_nesting_level {
            let (line, column) = self.get_current_position();
            return Err(ParseError::nesting_too_deep(
                self.max_nesting_level,
                line,
                column,
            ));
        }
        self.current_depth += 1;
        Ok(())
    }

    /// Decrease nesting depth counter
    fn decrease_depth(&mut self) {
        if self.current_depth > 0 {
            self.current_depth -= 1;
        }
    }

    /// Parse selector expression
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        let result = set::parse_set(self);

        // Ensure all tokens are consumed
        // All tokens except EOF should be consumed
        if result.is_ok() && self.current_token.is_some() {
            match &self.current_token {
                Some((Token::EOF, _, _)) => {
                    // EOF token is expected, consume it
                    self.read_token();
                }
                Some((token, line, column)) => {
                    // Non-EOF token means there's an error
                    return Err(ParseError::unexpected_token(
                        "EOF",
                        &format!("{}", token),
                        *line,
                        *column,
                    ));
                }
                None => {
                    // No more tokens, this is normal
                }
            }
        }

        result
    }

    /// Try to recover from an error
    fn try_recover(&mut self) -> Result<(), ParseError> {
        // Record initial parenthesis depth
        let mut depth = 0;

        while let Some((token, _, _)) = &self.current_token {
            match token {
                Token::LeftParen => depth += 1,
                Token::RightParen => {
                    if depth > 0 {
                        depth -= 1;
                    } else {
                        // Found an unmatched right parenthesis, might be a good sync point
                        self.read_token();
                        return Ok(());
                    }
                }
                Token::Pipeline | Token::Union | Token::Intersection | Token::Difference => {
                    if depth == 0 {
                        // Found a top-level operator, is a good sync point
                        return Ok(());
                    }
                }
                Token::EOF => return Ok(()),
                _ => {}
            }
            self.read_token();
        }

        Ok(())
    }

    
}

/// Convert lexer token stream to AST
pub fn parse(input: &str) -> Result<Node, ParseError> {
    let tokens = tokenize(input);

    let mut parser = Parser::new(tokens);

    let node = match parser.parse() {
        Ok(node) => Ok(node),
        Err(parse_error) => {
            if let Err(_) = parser.try_recover() {
                return Err(parse_error);
            }
            match set::parse_set(&mut parser) {
                Ok(recovered_node) => Ok(recovered_node),
                Err(_second_error) => {
                    Err(parse_error)
                }
            }
        }
    }?;

    let mut validator = SyntaxValidator::new();
    validator.validate(&node)?;

    println!("{:?}", node);

    Ok(node)
}
