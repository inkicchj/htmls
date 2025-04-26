use std::error::Error;
use std::fmt;

/// Parser error types
#[derive(Debug, Clone, PartialEq)]
pub enum ParseErrorKind {
    /// Unexpected token encountered
    UnexpectedToken,

    /// Invalid selector value
    InvalidSelectorValue,

    /// Nesting level too deep
    NestingTooDeep,

    /// Other syntax errors
    SyntaxError,
    
    /// Multiple text query directives
    MultipleTextSelectors,
    
    /// Element query directive after text query directive
    ElementAfterTextSelector,
}

impl fmt::Display for ParseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ParseErrorKind::InvalidSelectorValue => write!(f, "Invalid selector value"),
            ParseErrorKind::NestingTooDeep => write!(f, "Nesting level too deep"),
            ParseErrorKind::SyntaxError => write!(f, "Syntax error"),
            ParseErrorKind::MultipleTextSelectors => write!(f, "Multiple text query directives"),
            ParseErrorKind::ElementAfterTextSelector => write!(f, "Element query after text query"),
        }
    }
}

/// Parse error
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Error type
    pub kind: ParseErrorKind,

    /// Error message
    pub message: String,

    /// Line number where the error occurred
    pub line: usize,

    /// Column number where the error occurred
    pub column: usize,

    /// Error recovery hint
    pub recovery_hint: Option<String>,
}

impl ParseError {
    /// Create a new parse error
    #[allow(dead_code)]
    pub fn new(
        kind: ParseErrorKind,
        message: String,
        line: usize,
        column: usize,
        recovery_hint: Option<String>,
    ) -> Self {
        ParseError {
            kind,
            message,
            line,
            column,
            recovery_hint,
        }
    }

    /// Create an unexpected token error
    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::UnexpectedToken,
            message: format!("Expected {} but found {}", expected, found),
            line,
            column,
            recovery_hint: Some(format!("Please check if {} is missing here", expected)),
        }
    }

    pub fn syntax_error(msg: &str, line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::SyntaxError,
            message: msg.to_string(),
            line,
            column,
            recovery_hint: None,
        }
    }


    /// Create an invalid selector value error
    pub fn invalid_selector_value(value: &str, line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::InvalidSelectorValue,
            message: format!("Invalid selector value: {}", value),
            line,
            column,
            recovery_hint: Some("Please check if the selector value format is correct".to_string()),
        }
    }

    /// Create a nesting too deep error
    pub fn nesting_too_deep(max_depth: usize, line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::NestingTooDeep,
            message: format!("Expression nesting exceeds maximum depth ({})", max_depth),
            line,
            column,
            recovery_hint: Some("Please simplify the expression, reduce nesting levels".to_string()),
        }
    }
    
    /// Create a multiple text selectors error
    pub fn multiple_text_selectors(line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::MultipleTextSelectors,
            message: "Query operation can only contain one text query directive".to_string(),
            line,
            column,
            recovery_hint: Some("Please remove extra text query directives or separate them with set operations (+, *, -)".to_string()),
        }
    }
    
    /// Create an element after text selector error
    pub fn element_after_text_selector(line: usize, column: usize) -> Self {
        ParseError {
            kind: ParseErrorKind::ElementAfterTextSelector,
            message: "Element query directives cannot appear after text query directives".to_string(),
            line,
            column,
            recovery_hint: Some("Please place element query directives before text query directives".to_string()),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Parse error (line {}, column {}): {} - {}",
            self.line, self.column, self.kind, self.message
        )?;

        if let Some(hint) = &self.recovery_hint {
            write!(f, "\nHint: {}", hint)?;
        }

        Ok(())
    }
}

impl Error for ParseError {}
 