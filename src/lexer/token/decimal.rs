use super::{Token, TokenKind};

#[derive(Clone)]
pub struct Decimal {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
    pub value: f64,
}

impl Token for Decimal {
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

impl Decimal {
    pub fn value(&self) -> f64 {
        self.value
    }
}

impl From<f64> for Decimal {
    fn from(value: f64) -> Self {
        Self {
            kind: TokenKind::Decimal,
            value,
            ..Default::default()
        }
    }
}

impl Default for Decimal {
    fn default() -> Self {
        Self {
            kind: TokenKind::Decimal,
            line: usize::default(),
            column: usize::default(),
            value: f64::default(),
        }
    }
}
