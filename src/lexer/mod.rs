use crate::macros::bail;

use self::token::{
    Character, Decimal, Identifier, Integer, LexerError, StringLiteral, Token, TokenKind,
};

#[derive(PartialEq, Debug)]
enum TakeVariant {
    Take,
    Skip,
    Stop,
    Custom(char),
}

fn escape_character(ch: char) -> Result<char, LexerError> {
    match ch {
        't' => {
            return Ok('\t');
        }
        'n' => {
            return Ok('\n');
        }
        'r' => {
            return Ok('\r');
        }
        '"' => {
            return Ok('"');
        }
        '\'' => {
            return Ok('\'');
        }
        _ => bail!(LexerError::UnexpectedEscapeCharacter(ch)),
    }
}

pub mod token;

pub struct Lexer {
    pub tokens: Vec<Box<dyn Token>>,
    position: usize,
    line: usize,
    column: usize,
    source: String,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            position: 0,
            line: 1,
            column: 0,
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
        if let Some(ch) = self.source.chars().nth(last_position) {
            if ch != '\n' {
                self.column += 1;
            } else {
                self.line += 1;
                self.column = 0;
            }
        }
        return Ok(self.source.chars().nth(last_position));
    }

    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.position)
    }

    fn peek_ahead(&self) -> Option<char> {
        self.source.chars().nth(self.position + 1)
    }

    fn take_while(
        &mut self,
        filter: impl Fn(char, Option<char>) -> Result<TakeVariant, LexerError>,
    ) -> Result<String, LexerError> {
        let mut chars: String = String::new();
        let mut ch: Option<char> = None;
        loop {
            let prev = ch;
            ch = self.peek();
            if let Some(ch) = ch {
                if filter(ch, prev)? == TakeVariant::Stop {
                    break;
                }
            }
            let ch = self.next_char().unwrap_or(None);
            if let Some(ch) = ch {
                let res = filter(ch, prev)?;

                if let TakeVariant::Custom(cch) = res {
                    chars.push(cch);
                    continue;
                }

                if res == TakeVariant::Take {
                    chars.push(ch);
                } else if res != TakeVariant::Skip {
                    break;
                }
            } else {
                break;
            }
        }
        Ok(chars)
    }

    fn tokenize_ident(&mut self) -> Result<Identifier, LexerError> {
        // identifiers can't start with a number
        match self.peek() {
            Some(ch) if ch.is_digit(10) => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        let line = self.line;
        let column = self.column;
        let got = self.take_while(|ch, _prev| {
            if ch == '_' || ch.is_ascii_alphabetic() || ch.is_digit(10) {
                return Ok(TakeVariant::Take);
            }

            Ok(TakeVariant::Stop)
        })?;

        let mut tok = Identifier::from(got);
        tok.set_line(line);
        tok.set_column(column);
        Ok(tok)
    }

    fn tokenize_number(&mut self) -> Result<Box<dyn Token>, LexerError> {
        // number should start with a digit
        match self.peek() {
            Some(ch) if !ch.is_digit(10) => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        let line = self.line;
        let column = self.column;

        let got = self.take_while(|ch, _prev| {
            if ch.is_digit(10) || ch == '.' {
                return Ok(TakeVariant::Take);
            }

            Ok(TakeVariant::Stop)
        })?;

        // number can contain either 1 or zero points (dots)
        if got.matches('.').count() > 1 {
            bail!(LexerError::UnexpectedToken(got));
        }

        if got.matches('.').count() == 1 {
            let value: f64 = match got.parse() {
                Ok(num) => num,
                Err(_) => bail!(LexerError::ParseNumberError(got)),
            };

            let mut token = Decimal::from(value);
            token.set_line(line);
            token.set_column(column);

            return Ok(Box::new(token));
        } else {
            let value: usize = match got.parse() {
                Ok(num) => num,
                Err(_) => bail!(LexerError::ParseNumberError(got)),
            };

            let mut token = Integer::from(value);
            token.set_line(line);
            token.set_column(column);

            return Ok(Box::new(token));
        }
    }

    fn parse_comment(&mut self) -> Result<(), LexerError> {
        match self.peek() {
            Some(ch) if ch != '#' => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        self.take_while(|ch, _prev| {
            if ch != '\n' && ch != '\r' {
                return Ok(TakeVariant::Take);
            }

            Ok(TakeVariant::Stop)
        })?;

        Ok(())
    }

    fn tokenize_string_literal(&mut self) -> Result<Box<dyn Token>, LexerError> {
        match self.peek() {
            Some(ch) if ch != '"' => bail!(LexerError::UnexpectedToken(ch.to_string())),
            None => bail!(LexerError::UnexpectedEOF),
            _ => {}
        }

        let line = self.line;
        let column = self.column;

        self.next_char()?; // skip '"' character

        let got = self.take_while(|ch, prev| {
            if let Some(pch) = prev {
                if pch == '\\' {
                    let escape_ch = match escape_character(ch) {
                        Ok(ch) => ch,
                        Err(_) => bail!(LexerError::UnexpectedEscapeCharacter(ch)),
                    };
                    return Ok(TakeVariant::Custom(escape_ch));
                }
            }

            if ch == '\\' {
                return Ok(TakeVariant::Skip);
            }

            if ch != '"' {
                return Ok(TakeVariant::Take);
            }

            Ok(TakeVariant::Stop)
        })?;

        self.next_char()?; // skip '"' character (closing one)

        let mut token = StringLiteral::from(got);
        token.line = line;
        token.column = column;

        Ok(Box::new(token))
    }

    fn append_token(&mut self, mut token: Box<dyn Token>, add_position: Option<usize>) {
        let token_kind = token.kind();
        if let Some(add_position) = add_position {
            self.position += add_position;
            token.set_line(self.line);
            token.set_column(self.column);
            match token_kind {
                TokenKind::Newline => {
                    self.line += 1;
                    self.column = 0;
                }
                _ => {
                    self.column += 1;
                }
            }
        }
        self.tokens.push(token);
    }

    fn is_end(&self) -> bool {
        self.position >= self.source.chars().count()
    }

    pub fn tokenize(&mut self) -> Result<(), LexerError> {
        while !self.is_end() {
            let ch = self.peek();

            if let Some(ch) = ch {
                match ch {
                    '+' => {
                        if self.peek_ahead().is_some_and(|next_char| next_char == '+') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Increment)),
                                Some(2),
                            )
                        } else if self.peek_ahead().is_some_and(|next_char| next_char == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Addition)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Plus)), Some(1))
                        }
                    }
                    '-' => {
                        if self.peek_ahead().is_some_and(|next_char| next_char == '-') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Decrement)),
                                Some(2),
                            )
                        } else if self.peek_ahead().is_some_and(|nc| nc == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Subtraction)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Minus)), Some(1))
                        }
                    }
                    '*' => {
                        if self.peek_ahead().is_some_and(|nc| nc == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Multiplication)),
                                Some(2),
                            )
                        } else {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Multiply)),
                                Some(1),
                            )
                        }
                    }
                    '/' => {
                        if self.peek_ahead().is_some_and(|nc| nc == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Division)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Divide)), Some(1))
                        }
                    }
                    '=' => {
                        if self.peek_ahead().is_some_and(|next_char| next_char == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::IsEquals)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Equals)), Some(1))
                        }
                    }
                    '(' => {
                        self.append_token(Box::new(Character::from(TokenKind::OpenParen)), Some(1))
                    }
                    ')' => {
                        self.append_token(Box::new(Character::from(TokenKind::CloseParen)), Some(1))
                    }
                    '\n' => {
                        self.append_token(Box::new(Character::from(TokenKind::Newline)), Some(1))
                    }
                    '[' => self.append_token(
                        Box::new(Character::from(TokenKind::OpenSquareBracket)),
                        Some(1),
                    ),
                    ']' => self.append_token(
                        Box::new(Character::from(TokenKind::CloseSquareBracket)),
                        Some(1),
                    ),
                    '{' => self.append_token(
                        Box::new(Character::from(TokenKind::OpenCurlyBrace)),
                        Some(1),
                    ),
                    '}' => self.append_token(
                        Box::new(Character::from(TokenKind::CloseCurlyBrace)),
                        Some(1),
                    ),
                    ':' => self.append_token(Box::new(Character::from(TokenKind::Colon)), Some(1)),
                    '.' => self.append_token(Box::new(Character::from(TokenKind::Point)), Some(1)),
                    ',' => self.append_token(Box::new(Character::from(TokenKind::Comma)), Some(1)),
                    '%' => {
                        if self.peek_ahead().is_some_and(|nc| nc == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::Modulation)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Modulo)), Some(1))
                        }
                    }
                    '!' => {
                        if self.peek_ahead().is_some_and(|next_char| next_char == '=') {
                            self.append_token(
                                Box::new(Character::from(TokenKind::NotEquals)),
                                Some(2),
                            )
                        } else {
                            self.append_token(Box::new(Character::from(TokenKind::Not)), Some(1))
                        }
                    }
                    '<' => {
                        self.append_token(Box::new(Character::from(TokenKind::LessThan)), Some(1))
                    }
                    '>' => self
                        .append_token(Box::new(Character::from(TokenKind::GreaterThan)), Some(1)),
                    ' ' | '\r' => {
                        // ignore whitespaces
                        self.position += 1;
                        if ch.to_string().chars().count() > 0 {
                            self.column += 1;
                        }
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
                        } else if ch.is_alphabetic() {
                            let identifier = self.tokenize_ident()?;

                            let value = identifier.value();
                            match value.as_str() {
                                "fn" => {
                                    let mut token = Character::from(TokenKind::Fn);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "return" => {
                                    let mut token = Character::from(TokenKind::Return);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "if" => {
                                    let mut token = Character::from(TokenKind::If);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "else" => {
                                    let mut token = Character::from(TokenKind::Else);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "and" => {
                                    let mut token = Character::from(TokenKind::And);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "or" => {
                                    let mut token = Character::from(TokenKind::Or);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "for" => {
                                    let mut token = Character::from(TokenKind::For);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "in" => {
                                    let mut token = Character::from(TokenKind::In);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "let" => {
                                    let mut token = Character::from(TokenKind::Let);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "const" => {
                                    let mut token = Character::from(TokenKind::Const);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "class" => {
                                    let mut token = Character::from(TokenKind::Class);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "from" => {
                                    let mut token = Character::from(TokenKind::From);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "import" => {
                                    let mut token = Character::from(TokenKind::Import);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                "static" => {
                                    let mut token = Character::from(TokenKind::Static);
                                    token.set_line(identifier.line());
                                    token.set_column(identifier.column());
                                    self.append_token(Box::new(token), None);
                                    continue;
                                }
                                _ => {}
                            };

                            self.append_token(Box::new(identifier), None);
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

        let mut eof_token = Character::from(TokenKind::EOF);

        eof_token.set_line(self.line);
        eof_token.set_column(self.column);

        self.append_token(Box::new(eof_token), None);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tokenize_string(source: String) -> Vec<Box<dyn Token>> {
        let mut lexer = Lexer::new(source.clone());
        if lexer.tokenize().is_err() {
            panic!("Failed to tokenize source: {}", source);
        } else {
            return lexer.tokens;
        }
    }

    #[test]
    fn test_plus() {
        let tokens = tokenize_string("+".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Plus));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_minus() {
        let tokens = tokenize_string("-".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Minus));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_increment() {
        let tokens = tokenize_string("++".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Increment));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_decrement() {
        let tokens = tokenize_string("--".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Decrement));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_open_paren() {
        let tokens = tokenize_string("(".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::OpenParen));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_close_paren() {
        let tokens = tokenize_string(")".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::CloseParen));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_open_bracket() {
        let tokens = tokenize_string("[".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::OpenSquareBracket));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_close_bracket() {
        let tokens = tokenize_string("]".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::CloseSquareBracket));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_open_brace() {
        let tokens = tokenize_string("{".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::OpenCurlyBrace));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_close_brace() {
        let tokens = tokenize_string("}".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::CloseCurlyBrace));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_colon() {
        let tokens = tokenize_string(":".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Colon));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_comma() {
        let tokens = tokenize_string(",".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Comma));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_point() {
        let tokens = tokenize_string(".".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Point));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_multiply() {
        let tokens = tokenize_string("*".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Multiply));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_divide() {
        let tokens = tokenize_string("/".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Divide));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_modulo() {
        let tokens = tokenize_string("%".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Modulo));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_not() {
        let tokens = tokenize_string("!".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Not));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_greater_than() {
        let tokens = tokenize_string(">".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::GreaterThan));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_less_than() {
        let tokens = tokenize_string("<".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::LessThan));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_equals() {
        let tokens = tokenize_string("=".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Equals));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_is_equals() {
        let tokens = tokenize_string("==".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::IsEquals));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_not_equals() {
        let tokens = tokenize_string("!=".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::NotEquals));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_newline() {
        let tokens = tokenize_string("\n".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Newline));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_function() {
        let tokens: Vec<Box<dyn Token>> = tokenize_string("fn test(x, y) { return x }".to_string());
        let expected: Vec<TokenKind> = vec![
            TokenKind::Fn,
            TokenKind::Identifier,
            TokenKind::OpenParen,
            TokenKind::Identifier,
            TokenKind::Comma,
            TokenKind::Identifier,
            TokenKind::CloseParen,
            TokenKind::OpenCurlyBrace,
            TokenKind::Return,
            TokenKind::Identifier,
            TokenKind::CloseCurlyBrace,
            TokenKind::EOF,
        ];
        assert_eq!(tokens.len(), expected.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind(), *expected.get(i).unwrap())
        }
    }

    #[test]
    fn test_keyword_import() {
        let tokens = tokenize_string("import".to_string());
        assert_eq!(tokens.len(), 2);
        assert!(tokens
            .get(0)
            .is_some_and(|token| token.kind() == TokenKind::Import));
        assert!(tokens
            .get(1)
            .is_some_and(|token| token.kind() == TokenKind::EOF));
    }

    #[test]
    fn test_if_statement() {
        let tokens: Vec<Box<dyn Token>> = tokenize_string(
            "if true and 5 or 0 { something } else if { alternate } else { anything }".to_string(),
        );
        let expected: Vec<TokenKind> = vec![
            TokenKind::If,
            TokenKind::Identifier,
            TokenKind::And,
            TokenKind::Integer,
            TokenKind::Or,
            TokenKind::Integer,
            TokenKind::OpenCurlyBrace,
            TokenKind::Identifier,
            TokenKind::CloseCurlyBrace,
            TokenKind::Else,
            TokenKind::If,
            TokenKind::OpenCurlyBrace,
            TokenKind::Identifier,
            TokenKind::CloseCurlyBrace,
            TokenKind::Else,
            TokenKind::OpenCurlyBrace,
            TokenKind::Identifier,
            TokenKind::CloseCurlyBrace,
            TokenKind::EOF,
        ];
        assert_eq!(tokens.len(), expected.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind(), *expected.get(i).unwrap())
        }
    }

    #[test]
    fn test_for_loop() {
        let tokens: Vec<Box<dyn Token>> =
            tokenize_string("for i in [1, 2, 3] { print(i) }".to_string());
        let expected: Vec<TokenKind> = vec![
            TokenKind::For,
            TokenKind::Identifier,
            TokenKind::In,
            TokenKind::OpenSquareBracket,
            TokenKind::Integer,
            TokenKind::Comma,
            TokenKind::Integer,
            TokenKind::Comma,
            TokenKind::Integer,
            TokenKind::CloseSquareBracket,
            TokenKind::OpenCurlyBrace,
            TokenKind::Identifier,
            TokenKind::OpenParen,
            TokenKind::Identifier,
            TokenKind::CloseParen,
            TokenKind::CloseCurlyBrace,
            TokenKind::EOF,
        ];
        assert_eq!(tokens.len(), expected.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind(), *expected.get(i).unwrap())
        }
    }

    #[test]
    fn test_variable() {
        let tokens: Vec<Box<dyn Token>> = tokenize_string("let x = 5\nconst PI = 3.14".to_string());
        let expected: Vec<TokenKind> = vec![
            TokenKind::Let,
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::Integer,
            TokenKind::Newline,
            TokenKind::Const,
            TokenKind::Identifier,
            TokenKind::Equals,
            TokenKind::Decimal,
            TokenKind::EOF,
        ];
        assert_eq!(tokens.len(), expected.len());
        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(token.kind(), *expected.get(i).unwrap())
        }
    }
}
