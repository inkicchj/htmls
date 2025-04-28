use std::error::Error;
use std::fmt;

/// Interpreter error
#[derive(Debug, Clone)]
pub enum InterpreterError {
    /// HTML parsing error
    HtmlParseError(String),

    /// Syntax parsing error
    ParserError(String),

    /// Node selection error
    NodeSelectionError(String),

    /// Text extraction error
    TextExtractionError(String),

    /// Attribute extraction error
    AttributeExtractionError(String),

    /// Index out of bounds
    IndexOutOfBounds(usize, usize),

    /// Invalid step value
    InvalidStep(i64),

    /// Regular expression error
    InvalidRegex(String),

    /// Unknown function
    UnknownFunction(String),

    /// Missing argument
    MissingArgument(String),

    /// Invalid argument
    InvalidArgument(String),

    /// Execution error
    ExecutionError(String),

    /// Result limit exceeded
    ResultLimitExceeded(usize),
}

impl fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InterpreterError::HtmlParseError(msg) => write!(f, "HTML parsing error: {}", msg),
            InterpreterError::ParserError(msg) => write!(f, "Syntax parsing error: {}", msg),
            InterpreterError::NodeSelectionError(msg) => write!(f, "Node selection error: {}", msg),
            InterpreterError::TextExtractionError(msg) => write!(f, "Text extraction error: {}", msg),
            InterpreterError::AttributeExtractionError(msg) => write!(f, "Attribute extraction error: {}", msg),
            InterpreterError::IndexOutOfBounds(idx, len) => {
                write!(f, "Index out of bounds: index {} is out of range 0-{}", idx, len - 1)
            }
            InterpreterError::InvalidStep(step) => write!(f, "Invalid step: step cannot be {}", step),
            InterpreterError::InvalidRegex(msg) => write!(f, "Invalid regular expression: {}", msg),
            InterpreterError::UnknownFunction(name) => write!(f, "Unknown function: {}", name),
            InterpreterError::MissingArgument(msg) => write!(f, "Missing argument: {}", msg),
            InterpreterError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            InterpreterError::ExecutionError(msg) => write!(f, "Execution error: {}", msg),
            InterpreterError::ResultLimitExceeded(limit) => {
                write!(f, "Result limit exceeded: more than {} results", limit)
            }
        }
    }
}

impl Error for InterpreterError {}

// Implement conversion for regex errors
impl From<regex::Error> for InterpreterError {
    fn from(err: regex::Error) -> Self {
        InterpreterError::InvalidRegex(format!("{}", err))
    }
}

// Implement conversion for common I/O errors
impl From<std::io::Error> for InterpreterError {
    fn from(err: std::io::Error) -> Self {
        InterpreterError::ExecutionError(format!("IO error: {}", err))
    }
}


pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// Error helper methods
impl InterpreterError {
    /// Create a node selection error
    pub fn node_selection_error(message: impl Into<String>) -> Self {
        InterpreterError::NodeSelectionError(message.into())
    }

    /// Create an HTML parsing error
    pub fn html_parse_error(message: impl Into<String>) -> Self {
        InterpreterError::HtmlParseError(message.into())
    }

    /// Create a text extraction error
    pub fn text_extraction_error(message: impl Into<String>) -> Self {
        InterpreterError::TextExtractionError(message.into())
    }

    /// Create an attribute extraction error
    pub fn attribute_extraction_error(message: impl Into<String>) -> Self {
        InterpreterError::AttributeExtractionError(message.into())
    }

    /// Create an execution error
    pub fn execution_error(message: impl Into<String>) -> Self {
        InterpreterError::ExecutionError(message.into())
    }

    /// Create an unknown function error
    pub fn unknown_function(name: impl Into<String>) -> Self {
        InterpreterError::UnknownFunction(name.into())
    }

    /// Create a missing argument error
    pub fn missing_argument(message: impl Into<String>) -> Self {
        InterpreterError::MissingArgument(message.into())
    }

    /// Create an invalid argument error
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        InterpreterError::InvalidArgument(message.into())
    }
}