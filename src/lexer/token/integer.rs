use super::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Integer {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub value: usize,
}

impl Token for Integer {
    fn kind(&self) -> TokenKind {
        self.kind
    }

    fn line(&self) -> usize {
        self.line
    }

    fn column(&self) -> usize {
        self.column
    }

    fn set_line(&mut self, value: usize) {
        self.line = value;
    }

    fn set_column(&mut self, value: usize) {
        self.column = value;
    }
}

impl Integer {
    pub fn value(&self) -> usize {
        self.value
    }
}

impl From<usize> for Integer {
    fn from(value: usize) -> Self {
        Self {
            kind: TokenKind::Integer,
            value,
            ..Default::default()
        }
    }
}

impl Default for Integer {
    fn default() -> Self {
        Self {
            kind: TokenKind::Integer,
            line: usize::default(),
            column: usize::default(),
            value: usize::default(),
        }
    }
}
