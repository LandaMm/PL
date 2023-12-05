use crate::lexer::token::Tokens;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Tokens),
    UnexpectedEOF,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ParseError::UnexpectedToken(ref value) => {
                // TODO: add support for showing token value and position
                write!(f, "Unexpected token: '{:?}'", value)
            }
            ParseError::UnexpectedEOF => {
                write!(f, "Unexpected end of file")
            }
        }
    }
}
