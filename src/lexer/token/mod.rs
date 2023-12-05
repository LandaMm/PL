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

pub mod character;
pub mod decimal;
pub mod identifier;
pub mod integer;
pub mod string_literal;

use std::fmt::Debug;

pub use character::*;
pub use decimal::*;
pub use identifier::*;
pub use integer::*;
pub use string_literal::*;

pub trait Token {
    fn kind(&self) -> TokenKind;
    fn line(&self) -> usize;
    fn column(&self) -> usize;

    fn set_line(&mut self, value: usize);
    fn set_column(&mut self, value: usize);
}

impl Debug for dyn Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Token({:?}, {}:{})",
            self.kind(),
            self.line(),
            self.column()
        )
    }
}

pub struct SimpleToken {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Identifier,
    Integer,
    Decimal,
    StringLiteral,
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
    Point,
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
    Let,
    Const,
    Class,
    From,
    // Another required tokens for parser
    EOF, // signified the end of file.
}