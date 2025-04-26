use std::error::Error;
use std::fmt;


#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // pipe operator
    Pipeline, // >

    // element selection instruction
    Class, // class
    Id,    // id
    Tag,   // tag
    Attr,  // attr

    // text selection instruction
    Text, // text
    Src,  // src
    Href, // href

    // function call
    Function(String), // @name
    // function parameter separator
    Comma, // ,

    // list element selection
    Colon,         // : 
    Number(usize),

    // selector argument
    Argument(String), // common argument
    QuotedArgument(String), // "quoted argument"

    // regular expression Ssymbol
    Regex, // ~


    LeftParen,  // (
    RightParen, // )

    // union operator
    Union, // |
    // intersection operator
    Intersection, // &
    // difference operator
    Difference, // ^

    EOF,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Pipeline => write!(f, ">"),
            Token::Class => write!(f, "class"),
            Token::Id => write!(f, "id"),
            Token::Tag => write!(f, "tag"),
            Token::Attr => write!(f, "attr"),
            Token::Text => write!(f, "text"),
            Token::Src => write!(f, "src"),
            Token::Href => write!(f, "href"),
            Token::Argument(arg) => write!(f, "{}", arg),
            Token::QuotedArgument(arg) => write!(f, "\"{}\"", arg),
            Token::Regex => write!(f, "~"),
            Token::Function(func) => write!(f, "@{}", func),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::Number(n) => write!(f, "{}", n),
            Token::LeftParen => write!(f, "("),
            Token::RightParen => write!(f, ")"),
            Token::Union => write!(f, "|"),
            Token::Intersection => write!(f, "&"),
            Token::Difference => write!(f, "^"),
            Token::EOF => write!(f, "EOF"),
        }
    }
}


#[derive(Debug)]
pub struct LexerError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexical error(line {}, column {}): {}",
            self.line, self.column, self.message
        )
    }
}

impl Error for LexerError {}

/// lexical analyzer
pub struct Lexer {
    // character buffer
    chars: Vec<char>,
    // current processing location
    position: usize,
    // pre-read position
    read_position: usize,
    // current character
    current_char: Option<char>,
    // current line number
    line: usize,
    // current column number
    column: usize, 
}

impl Lexer {

    pub fn new(input: &str) -> Self {

        let estimated_capacity = input.len() + 1;
        let mut chars = Vec::with_capacity(estimated_capacity);
        chars.extend(input.chars());

        let mut lexer = Lexer {
            chars,
            position: 0,
            read_position: 0,
            current_char: None,
            line: 1,
            column: 0,
        };

        lexer.read_char();
        lexer
    }

    /// Read the next character and update the position information.
    fn read_char(&mut self) {
        if self.read_position >= self.chars.len() {
            self.current_char = None;
        } else {
            self.current_char = Some(self.chars[self.read_position]);
        }

        self.position = self.read_position;
        self.read_position += 1;

        if let Some('\n') = self.current_char {
            self.line += 1;
            self.column = 0;
        } else {
            self.column += 1;
        }
    }

    /// Skip whitespace characters.
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    /// Determine if a character is a valid starting character for an identifier.
    fn is_identifier_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || Self::is_unicode_identifier_part(c)
    }

    /// Determine if a character is a valid part of an identifier.
    fn is_identifier_part(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_' || Self::is_unicode_identifier_part(c)
    }

    /// Determine whether it is a supported Unicode identifier character.
    fn is_unicode_identifier_part(c: char) -> bool {
        (c >= '\u{4E00}' && c <= '\u{9FFF}') ||
        (c >= '\u{3040}' && c <= '\u{309F}') ||
        (c >= '\u{30A0}' && c <= '\u{30FF}') ||
        (c >= '\u{AC00}' && c <= '\u{D7AF}') ||
        (c >= '\u{1F600}' && c <= '\u{1F64F}') ||
        (c >= '\u{1F300}' && c <= '\u{1F5FF}') ||
        (c >= '\u{1F680}' && c <= '\u{1F6FF}') ||
        (c >= '\u{2600}' && c <= '\u{26FF}')
    }

    /// Determine if a character is a valid starting character for a function name.
    fn is_function_name_start(&self, c: char) -> bool {
        c.is_ascii_alphabetic()
    }

    /// Determine if a character is a valid part of a function name.
    fn is_function_name_part(&self, c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    /// Get the next token.
    pub fn next_token(&mut self) -> Result<Token, LexerError> {

        self.skip_whitespace();

        if self.current_char.is_none() {
            return Ok(Token::EOF);
        }

        match self.current_char.unwrap() {
            '>' => {
                self.read_char();
                Ok(Token::Pipeline)
            }
            ',' => {
                self.read_char();
                Ok(Token::Comma)
            }
            ':' => {
                self.read_char();
                Ok(Token::Colon)
            }
            '|' => {
                self.read_char();
                Ok(Token::Union)
            }
            '^' => {
                self.read_char();
                Ok(Token::Difference)
            }
            '&' => {
                self.read_char();
                Ok(Token::Intersection)
            }
            '@' => self.read_function(),
            '"' => self.read_quoted_string(),
            '~' => {
                self.read_char();
                Ok(Token::Regex)
            }
            '(' => {
                self.read_char();
                Ok(Token::LeftParen)
            }
            ')' => {
                self.read_char();
                Ok(Token::RightParen)
            }
            '0'..='9' => self.read_number(),
            _ => self.read_argument(),
        }
    }

    /// Read numbers.
    fn read_number(&mut self) -> Result<Token, LexerError> {
        let start_position = self.position;

        while let Some(c) = self.current_char {
            if c.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        let number_str: String = self.chars[start_position..self.position].iter().collect();

        match number_str.parse::<usize>() {
            Ok(number) => Ok(Token::Number(number)),
            Err(_) => Err(LexerError {
                message: format!("Unable to resolve the number: {}", number_str),
                line: self.line,
                column: self.column,
            }),
        }
    }

    /// Read identifiers (keywords such as class, id, etc.)
    #[deprecated(note = "Keyword analysis has been added to the function for reading argument.")]
    #[allow(dead_code)]
    fn read_identifier(&mut self) -> Result<Token, LexerError> {
        let start_position = self.position;

        while let Some(c) = self.current_char {
            if self.is_identifier_part(c) {
                self.read_char();
            } else {
                break;
            }
        }

        let identifier: String = self.chars[start_position..self.position].iter().collect();

        match identifier.as_str() {
            "class" => Ok(Token::Class),
            "id" => Ok(Token::Id),
            "tag" => Ok(Token::Tag),
            "attr" => Ok(Token::Attr),
            "text" => Ok(Token::Text),
            "src" => Ok(Token::Src),
            "href" => Ok(Token::Href),
            _ => Err(LexerError {
                message: "Illegal identifier".to_string(),
                line: self.line,
                column: self.column,
            }),
        }
    }

    /// Read the function name (the part after @)
    fn read_function(&mut self) -> Result<Token, LexerError> {

        self.read_char();

        let start_position = self.position;

        if let Some(c) = self.current_char {
            if !self.is_function_name_start(c) {
                return Err(LexerError {
                    message: "Function names must start with a letter.".to_string(),
                    line: self.line,
                    column: self.column,
                });
            }
        } else {
            return Err(LexerError {
                message: "Function name cannot be empty.".to_string(),
                line: self.line,
                column: self.column,
            });
        }

        while let Some(c) = self.current_char {
            if self.is_function_name_part(c) {
                self.read_char();
            } else {
                break;
            }
        }

        let function_name: String = self.chars[start_position..self.position].iter().collect();

        Ok(Token::Function(function_name))
    }

    /// Read a quoted string.
    fn read_quoted_string(&mut self) -> Result<Token, LexerError> {
   
        self.read_char();

        let mut value = String::new();
        let mut escaped = false;


        while let Some(c) = self.current_char {
            if escaped {
                match c {
                    '"' => value.push('"'),
                    '\\' => value.push('\\'),
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    'u' => {
                        let mut unicode_value = String::new();
                        for _ in 0..4 {
                            self.read_char();
                            if let Some(hex_char) = self.current_char {
                                if hex_char.is_ascii_hexdigit() {
                                    unicode_value.push(hex_char);
                                } else {
                                    return Err(LexerError {
                                        message: format!(
                                            "Invalid Unicode escape sequence: \\u{}",
                                            unicode_value
                                        ),
                                        line: self.line,
                                        column: self.column,
                                    });
                                }
                            } else {
                                return Err(LexerError {
                                    message: "Unfinished Unicode escape sequence.".to_string(),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                        }

                        // Convert hexadecimal values to Unicode characters
                        if let Ok(code_point) = u32::from_str_radix(&unicode_value, 16) {
                            if let Some(unicode_char) = std::char::from_u32(code_point) {
                                value.push(unicode_char);
                            } else {
                                return Err(LexerError {
                                    message: format!("Invalid Unicode code point: U+{}", unicode_value),
                                    line: self.line,
                                    column: self.column,
                                });
                            }
                        } else {
                            return Err(LexerError {
                                message: format!("Unable to resolve Unicode escape sequence: \\u{}", unicode_value),
                                line: self.line,
                                column: self.column,
                            });
                        }
                    }
                    _ => value.push(c),
                }
                escaped = false;
                self.read_char();
            } else if c == '\\' {
                escaped = true;
                self.read_char();
            } else if c == '"' {
                self.read_char();
                return Ok(Token::QuotedArgument(value));
            } else {
                value.push(c);
                self.read_char();
            }
        }

        Err(LexerError {
            message: "Unterminated string.".to_string(),
            line: self.line,
            column: self.column,
        })
    }

    /// Read normal parameters
    fn read_argument(&mut self) -> Result<Token, LexerError> {
        let start_position = self.position;

        while let Some(c) = self.current_char {
            if c.is_whitespace() || c == '>' || c == ',' || c == '"' || c == '@' || c == ':' {
                break;
            }
            self.read_char();
        }

        let argument: String = self.chars[start_position..self.position].iter().collect();

        if argument.is_empty() {
            return Err(LexerError {
                message: format!("Unrecognized characters: {:?}", self.current_char),
                line: self.line,
                column: self.column,
            });
        }

        // check if it is a keyword
        match argument.as_str() {
            "class" => Ok(Token::Class),
            "id" => Ok(Token::Id),
            "tag" => Ok(Token::Tag),
            "attr" => Ok(Token::Attr),
            "text" => Ok(Token::Text),
            "src" => Ok(Token::Src),
            "href" => Ok(Token::Href),
            _ => Ok(Token::Argument(argument))
        }
    }

    fn recover_from_error(&mut self) {
        while let Some(c) = self.current_char {
            if c == '>' || c == ',' || c == '"' || c == '@' || self.is_identifier_start(c) {
                break;
            }
            self.read_char();
        }
    }
}


pub fn tokenize(input: &str) -> Vec<(Token, usize, usize)> {
    let mut lexer = Lexer::new(input);

    let estimated_tokens = (input.len() / 4).max(8);
    let mut tokens_with_pos = Vec::with_capacity(estimated_tokens);

    loop {
        let line = lexer.line;
        let column = lexer.column;

        match lexer.next_token() {
            Ok(Token::EOF) => {
                tokens_with_pos.push((Token::EOF, line, column));
                break;
            }
            Ok(token) => tokens_with_pos.push((token, line, column)),
            Err(_) => lexer.recover_from_error(),
        }
    }

    tokens_with_pos
}
