use crate::{lexer::token::TokenKind, Node};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(TokenKind, usize, usize), // token_kind, line column
    ConstantNotInitialized(String, usize, usize), // variable_name, line, column
    InvalidFunctionName(Box<Node>),
    UnexpectedEOF,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken(kind, line, column) => {
                // println!("{:?} {}:{}", kind, line, column);
                // TODO: add support for showing token value
                write!(
                    f,
                    "Unexpected token: '{:?}' at position {}:{}",
                    kind, line, column
                )
            }
            ParseError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
            ParseError::ConstantNotInitialized(variable_name, line, column) => {
                write!(
                    f,
                    "The constant {} must be initialized at {}:{}",
                    variable_name, line, column
                )
            }
            ParseError::InvalidFunctionName(node) => {
                write!(
                    f,
                    "Unexpected name of the function: {node:?}. Expected identifier."
                )
            }
        }
    }
}
