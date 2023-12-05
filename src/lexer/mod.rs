use crate::macros::bail;

use self::token::{LexerError, Tokens};

pub mod token;

pub struct Lexer {
    pub tokens: Vec<Tokens>,
    position: usize,
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            position: 0,
            tokens: vec![],
            source,
        }
    }

    fn next_char(&mut self) -> Result<Option<char>, LexerError> {
        if self.source.chars().nth(self.position).is_none() {
            bail!(LexerError::UnexpectedEOF)
        }
        let last_position = self.position.clone();
        self.position += 1;
        return Ok(self.source.chars().nth(last_position));
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    fn take_while(&mut self, filter: impl Fn(char) -> bool) -> Result<String, LexerError> {
        let mut chars: String = String::new();
        loop {
            let ch = self.peek();
            if let Some(ch) = ch {
                if !filter(ch) {
                    break;
                }
            }
            let ch = self.next_char().unwrap_or(None);
            if let Some(ch) = ch {
                if filter(ch) {
                    chars.push(ch);
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(chars)
    }

    fn tokenize_ident(&mut self) -> Result<Tokens, LexerError> {
        // identifiers can't start with a number
        match self.peek() {
            Some(ch) if ch.is_digit(10) => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        let got = self.take_while(|ch| ch == '_' || ch.is_ascii_alphabetic() || ch.is_digit(10))?;

        let tok = Tokens::Identifier(got);
        Ok(tok)
    }

    fn tokenize_number(&mut self) -> Result<Tokens, LexerError> {
        // number should start with a digit
        match self.peek() {
            Some(ch) if !ch.is_digit(10) => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        let got = self.take_while(|ch| ch.is_digit(10) || ch == '.')?;

        // number can contain either 1 or zero points (dots)
        if got.matches('.').count() > 1 {
            bail!(LexerError::UnexpectedToken(got));
        }

        if got.matches('.').count() == 1 {
            let value: f64 = match got.parse() {
                Ok(num) => num,
                Err(_) => bail!(LexerError::ParseNumberError(got)),
            };

            return Ok(Tokens::Decimal(value));
        } else {
            let value: usize = match got.parse() {
                Ok(num) => num,
                Err(_) => bail!(LexerError::ParseNumberError(got)),
            };

            return Ok(Tokens::Integer(value));
        }
    }

    fn parse_comment(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            Some(ch) if ch != '#' => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        self.take_while(|ch| ch != '\n' && ch != '\r')?;

        Ok(())
    }

    fn tokenize_string_literal(&mut self) -> Result<Tokens, LexerError> {
        match self.peek() {
            Some(ch) if ch != '"' => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        self.next_char()?; // skip '"' character

        let got = self.take_while(|ch| ch != '"')?;

        self.next_char()?; // skip '"' character (closing one)

        Ok(Tokens::StringLiteral(got))
    }

    fn append_token(&mut self, token: Tokens, add_position: Option<usize>) {
        self.tokens.push(token);
        if let Some(add_position) = add_position {
            self.position += add_position;
        }
    }

    fn is_end(&self) -> bool {
        self.position >= self.source.len()
    }

    pub fn tokenize(&mut self) -> Result<(), LexerError> {
        while !self.is_end() {
            let ch = self.peek();

            if let Some(ch) = ch {
                match ch {
                    '+' => self.append_token(Tokens::Plus, Some(1)),
                    '-' => self.append_token(Tokens::Minus, Some(1)),
                    '*' => self.append_token(Tokens::Multiply, Some(1)),
                    '/' => self.append_token(Tokens::Divide, Some(1)),
                    '=' => self.append_token(Tokens::Equals, Some(1)),
                    '(' => self.append_token(Tokens::OpenParen, Some(1)),
                    ')' => self.append_token(Tokens::CloseParen, Some(1)),
                    '\n' => self.append_token(Tokens::Newline, Some(1)),
                    '[' => self.append_token(Tokens::OpenSquareBracket, Some(1)),
                    ']' => self.append_token(Tokens::CloseSquareBracket, Some(1)),
                    '{' => self.append_token(Tokens::OpenCurlyBrace, Some(1)),
                    '}' => self.append_token(Tokens::CloseCurlyBrace, Some(1)),
                    ':' => self.append_token(Tokens::Colon, Some(1)),
                    ',' => self.append_token(Tokens::Comma, Some(1)),
                    '%' => self.append_token(Tokens::Modulo, Some(1)),
                    '!' => self.append_token(Tokens::Not, Some(1)),
                    '<' => self.append_token(Tokens::LessThan, Some(1)),
                    '>' => self.append_token(Tokens::GreaterThan, Some(1)),
                    ' ' | '\r' => {
                        // ignore whitespaces
                        self.position += 1;
                    }
                    '#' => self.parse_comment()?,
                    '"' => {
                        let string_literal = self.tokenize_string_literal()?;
                        self.append_token(string_literal, None);
                    }
                    ch => {
                        if ch.is_digit(10) {
                            let number = self.tokenize_number()?;
                            self.append_token(number, None);
                            continue;
                        } else if ch.is_ascii_alphabetic() {
                            let identifier = self.tokenize_ident()?;

                            match identifier.clone() {
                                Tokens::Identifier(value) => match value.as_str() {
                                    "def" => {
                                        self.append_token(Tokens::Def, None);
                                        continue;
                                    }
                                    "true" => {
                                        self.append_token(Tokens::True, None);
                                        continue;
                                    }
                                    "false" => {
                                        self.append_token(Tokens::False, None);
                                        continue;
                                    }
                                    "return" => {
                                        self.append_token(Tokens::Return, None);
                                        continue;
                                    }
                                    "if" => {
                                        self.append_token(Tokens::If, None);
                                        continue;
                                    }
                                    "else" => {
                                        self.append_token(Tokens::Else, None);
                                        continue;
                                    }
                                    "and" => {
                                        self.append_token(Tokens::And, None);
                                        continue;
                                    }
                                    "or" => {
                                        self.append_token(Tokens::Or, None);
                                        continue;
                                    }
                                    "for" => {
                                        self.append_token(Tokens::For, None);
                                        continue;
                                    }
                                    "in" => {
                                        self.append_token(Tokens::In, None);
                                        continue;
                                    }
                                    "let" => {
                                        self.append_token(Tokens::Let, None);
                                        continue;
                                    }
                                    "const" => {
                                        self.append_token(Tokens::Const, None);
                                        continue;
                                    }
                                    _ => {}
                                },
                                _ => {}
                            }

                            self.append_token(identifier, None);
                            continue;
                        } else {
                            bail!(LexerError::UnexpectedToken(ch.to_string()))
                        }
                    }
                }
            } else {
                bail!(LexerError::UnexpectedEOF)
            }
        }

        self.append_token(Tokens::EOF, None);

        Ok(())
    }
}
