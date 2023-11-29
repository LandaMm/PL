// let x = 10 + 5 - (8 * 2) / 5.6
// [
//  identifier(let),
//  identifier(x),
//  equals,
//  integer(10),
//  plus,
//  integer(5),
//  minus,
//  open_paren,
//  integer(8),
//  multiply,
//  integer(2),
//  close_paren,
//  divide,
//  decimal(5.6)
// ]

#[derive(Debug)]
pub enum LexerError {
    UnexpectedToken(String),
    ParseNumberError(String),
    UnexpectedEOF,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            LexerError::UnexpectedEOF => write!(f, "Unexpected EOF"),
            LexerError::UnexpectedToken(ref value) => {
                write!(f, "Unexpected token: '{}'", value)
            }
            LexerError::ParseNumberError(ref value) => {
                write!(f, "Failed to parse number: '{}'", value)
            }
        }
    }
}

impl std::error::Error for LexerError {}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Integer(usize),
    Decimal(f64),
    StringLiteral(String),
    Plus,
    Minus,
    OpenParen,
    CloseParen,
    OpenSquareBracket,
    CloseSquareBracket,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Colon,
    Comma,
    Multiply,
    Divide,
    Modulo,
    Not,
    GreaterThan,
    LessThan,
    Equals,
    // Whitespace, ignore, cz whitespace is not getting used as syntax part
    Newline,
    // Keywords
    Def,
    True,
    False,
    Return,
    If,
    Else,
    And,
    Or,
    For,
    In,
    // Another required tokens for parser
    EOF, // signified the end of file.
}

impl From<String> for Token {
    fn from(other: String) -> Self {
        Token::Identifier(other.to_string())
    }
}

impl<'a> From<&'a str> for Token {
    fn from(other: &'a str) -> Self {
        Token::Identifier(other.to_string())
    }
}

impl From<usize> for Token {
    fn from(other: usize) -> Self {
        Token::Integer(other)
    }
}

impl From<f64> for Token {
    fn from(other: f64) -> Self {
        Token::Decimal(other)
    }
}
