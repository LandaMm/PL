use crate::{lexer::token::Token, macros::bail};

use super::{
    error::ParseError,
    nodes::{BinaryOperator, Node},
};

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    pub fn produce_ast(&mut self) -> Result<Node, ParseError> {
        let mut program = Node::Program(vec![]);

        while self.not_eof() {
            let stmt = self.statement()?;
            if let Node::Program(ref mut stmts) = program {
                stmts.push(Box::new(stmt));
            }
        }

        Ok(program)
    }

    fn not_eof(&self) -> bool {
        let current_token = self.tokens.get(self.position);
        current_token.is_some_and(|token| !matches!(token, Token::EOF))
    }

    fn get_current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap()
    }

    fn eat(&mut self) -> Result<&Token, ParseError> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        match token {
            Some(token) => Ok(token),
            None => bail!(ParseError::UnexpectedEOF),
        }
    }

    fn statement(&mut self) -> Result<Node, ParseError> {
        // skip to parse_expr
        self.expression()
    }

    fn expression(&mut self) -> Result<Node, ParseError> {
        let mut result = self.term()?;

        while matches!(self.get_current_token().clone(), Token::Plus)
            || matches!(self.get_current_token().clone(), Token::Minus)
        {
            let current_token = self.get_current_token().clone();
            self.eat()?;

            match current_token {
                Token::Plus => {
                    let right = self.term()?;

                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::Plus,
                        Box::new(right),
                    );
                }
                Token::Minus => {
                    let right = self.term()?;

                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::Minus,
                        Box::new(right),
                    );
                }
                _ => bail!(ParseError::UnexpectedToken(current_token.clone())),
            }
        }

        Ok(result)
    }

    fn term(&mut self) -> Result<Node, ParseError> {
        let mut left = self.factor()?;

        while matches!(self.get_current_token().clone(), Token::Multiply)
            || matches!(self.get_current_token().clone(), Token::Divide)
            || matches!(self.get_current_token().clone(), Token::Modulo)
        {
            let current_token = self.get_current_token().clone();
            self.eat()?;

            match current_token {
                Token::Multiply => {
                    let right = self.factor()?;

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Multiply,
                        Box::new(right),
                    );
                }
                Token::Divide => {
                    let right = self.factor()?;

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Divide,
                        Box::new(right),
                    );
                }
                Token::Modulo => {
                    let right = self.factor()?;

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Modulo,
                        Box::new(right),
                    );
                }
                _ => bail!(ParseError::UnexpectedToken(current_token.clone())),
            }
        }

        Ok(left)
    }

    fn factor(&mut self) -> Result<Node, ParseError> {
        let current_token = self.get_current_token().clone();

        match current_token {
            Token::Identifier(identifier) => {
                self.eat()?;
                Ok(Node::Identifier(identifier.to_owned()))
            }
            Token::Integer(value) => {
                self.eat()?;
                Ok(Node::IntegerLiteral(value.to_owned()))
            }
            Token::Decimal(value) => {
                self.eat()?;
                Ok(Node::DecimalLiteral(value.to_owned()))
            }
            Token::OpenParen => {
                self.eat()?; // eat open paren
                let expr = self.expression()?;
                self.eat()?; // eat close paren
                Ok(expr)
            }
            token => bail!(ParseError::UnexpectedToken(token.to_owned())),
        }
    }
}
