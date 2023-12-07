use super::{Token, TokenKind};

#[derive(Clone)]
pub struct StringLiteral {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub value: String,
}

impl Token for StringLiteral {
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

impl StringLiteral {
    pub fn value(&self) -> String {
        self.value.clone()
    }
}

impl From<String> for StringLiteral {
    fn from(value: String) -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            value,
            ..Default::default()
        }
    }
}

impl Default for StringLiteral {
    fn default() -> Self {
        Self {
            kind: TokenKind::StringLiteral,
            line: usize::default(),
            column: usize::default(),
            value: String::default(),
        }
    }
}
