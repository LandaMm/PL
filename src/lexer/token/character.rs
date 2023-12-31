use super::{Token, TokenKind};

#[derive(Clone)]
pub struct Character {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token for Character {
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

    fn into_any(self: Box<Self>) -> Box<dyn std::any::Any> {
        self
    }
}

impl From<TokenKind> for Character {
    fn from(value: TokenKind) -> Self {
        Self {
            kind: value,
            ..Default::default()
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            kind: TokenKind::EOF,
            line: usize::default(),
            column: usize::default(),
        }
    }
}
