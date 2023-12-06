use super::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub value: String,
}

impl Token for Identifier {
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

impl Identifier {
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Self {
            kind: TokenKind::Identifier,
            value,
            ..Default::default()
        }
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Self {
            kind: TokenKind::Identifier,
            line: usize::default(),
            column: usize::default(),
            value: String::default(),
        }
    }
}
