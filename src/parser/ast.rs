use crate::{
    lexer::token::{Decimal, Identifier, Integer, StringLiteral, Token, TokenKind},
    macros::bail,
};

use std::any::Any;

use super::{
    error::ParseError,
    nodes::{BinaryOperator, LogicalOperator, Node, UnaryOperator},
};

pub struct Parser {
    tokens: Vec<Box<dyn Token>>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Box<dyn Token>>) -> Self {
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
        current_token.is_some_and(|token| !matches!(token.kind(), TokenKind::EOF))
    }

    fn get_current_token(&self) -> Result<&Box<dyn Token>, ParseError> {
        if let Some(token) = self.tokens.get(self.position) {
            Ok(token)
        } else {
            bail!(ParseError::UnexpectedEOF)
        }
    }

    fn eat(&mut self, kind: TokenKind) -> Result<&Box<dyn Token>, ParseError> {
        let token = self.tokens.get(self.position);
        self.position += 1;
        if token.is_none() {
            bail!(ParseError::UnexpectedEOF)
        }

        let token = token.unwrap();
        let token_data = token;
        let token_kind = token_data.kind();
        let line = token_data.line();
        let column = token_data.column();

        if token_kind == kind {
            Ok(token)
        } else {
            bail!(ParseError::UnexpectedToken(token_kind, line, column))
        }
    }

    fn statement(&mut self) -> Result<Node, ParseError> {
        match self.get_current_token()?.kind() {
            TokenKind::Let | TokenKind::Const => self.variable_declaration(),
            TokenKind::Def => self.function_declaration(),
            TokenKind::If => self.if_statement(),
            _ => self.binary_expression(),
        }
    }

    fn function_declaration(&mut self) -> Result<Node, ParseError> {
        self.eat(TokenKind::Def)?;

        let id = Box::new(self.identifier()?);

        self.eat(TokenKind::OpenParen)?;

        let mut params: Vec<Box<Node>> = vec![];

        while self.get_current_token()?.kind() != TokenKind::CloseParen {
            params.push(Box::new(self.identifier()?));
        }

        self.eat(TokenKind::CloseParen)?;

        let body = Box::new(self.block_statement()?);

        Ok(Node::FunctionDeclaration(id, params, body))
    }

    fn block_statement(&mut self) -> Result<Node, ParseError> {
        self.eat(TokenKind::OpenCurlyBrace)?;

        let mut statements: Vec<Box<Node>> = vec![];

        while self.get_current_token()?.kind() != TokenKind::CloseCurlyBrace {
            statements.push(Box::new(self.statement()?));
        }

        self.eat(TokenKind::CloseCurlyBrace)?;

        Ok(Node::BlockStatement(statements))
    }

    fn if_statement(&mut self) -> Result<Node, ParseError> {
        self.eat(TokenKind::If)?;

        let condition = self.binary_expression()?;

        let consequent = self.block_statement()?;

        let mut alternate: Option<Box<Node>> = None;

        if self.get_current_token()?.kind() == TokenKind::Else {
            self.eat(TokenKind::Else)?;

            if self.get_current_token()?.kind() == TokenKind::If {
                alternate = Some(Box::new(self.if_statement()?));
            } else {
                let block = self.block_statement()?;
                alternate = Some(Box::new(block));
            }
        }

        Ok(Node::IfStatement(
            Box::new(condition),
            Box::new(consequent),
            alternate,
        ))
    }

    fn variable_declaration(&mut self) -> Result<Node, ParseError> {
        let is_constant = if matches!(self.get_current_token()?.kind(), TokenKind::Const) {
            true
        } else {
            false
        };
        if is_constant {
            self.eat(TokenKind::Const)?;
        } else {
            self.eat(TokenKind::Let)?;
        }
        let identifier_token = self.eat(TokenKind::Identifier)?;
        let kind = identifier_token.kind().clone();
        let line = identifier_token.line().clone();
        let column = identifier_token.column().clone();
        let identifier_token = dyn_clone::clone_box(&**identifier_token);
        let identifier = identifier_token.into_any().downcast::<Identifier>();
        if identifier.is_err() {
            bail!(ParseError::UnexpectedToken(kind, line, column))
        }
        let identifier = identifier.unwrap();
        // check if we have some value to assign
        if self.get_current_token()?.kind() == TokenKind::Equals {
            self.eat(TokenKind::Equals)?;
            let value = self.binary_expression()?;
            Ok(Node::VariableDeclaration(
                identifier.value(),
                Some(Box::new(value)),
                is_constant,
            ))
        } else {
            // check if variable was a constant
            if is_constant {
                // constant cannot be declarated without value
                bail!(ParseError::ConstantNotInitialized(
                    identifier.value(),
                    line,
                    column
                ))
            }
            Ok(Node::VariableDeclaration(
                identifier.value(),
                None,
                is_constant,
            ))
        }
    }

    fn binary_expression(&mut self) -> Result<Node, ParseError> {
        let mut result = self.additive_expression()?;

        while self.get_current_token()?.kind() == TokenKind::LessThan
            || self.get_current_token()?.kind() == TokenKind::GreaterThan
            || self.get_current_token()?.kind() == TokenKind::IsEquals
            || self.get_current_token()?.kind() == TokenKind::NotEquals
        {
            let token = self.eat(self.get_current_token()?.kind())?;

            match token.kind() {
                TokenKind::LessThan => {
                    let right = self.additive_expression()?;
                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::LessThan,
                        Box::new(right),
                    );
                }
                TokenKind::GreaterThan => {
                    let right = self.additive_expression()?;
                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::GreaterThan,
                        Box::new(right),
                    );
                }
                TokenKind::IsEquals => {
                    let right = self.additive_expression()?;
                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::IsEquals,
                        Box::new(right),
                    );
                }
                TokenKind::NotEquals => {
                    let right = self.additive_expression()?;
                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::NotEquals,
                        Box::new(right),
                    );
                }
                _ => {}
            }
        }

        // check if it logical expressions, e.g. we have && as current token
        while self.get_current_token()?.kind() == TokenKind::And
            || self.get_current_token()?.kind() == TokenKind::Or
        {
            let token = self.eat(self.get_current_token()?.kind())?;

            match token.kind() {
                TokenKind::And => {
                    let right = self.binary_expression()?;
                    result = Node::LogicalExpression(
                        Box::new(result),
                        LogicalOperator::And,
                        Box::new(right),
                    );
                }
                TokenKind::Or => {
                    let right = self.binary_expression()?;
                    result = Node::LogicalExpression(
                        Box::new(result),
                        LogicalOperator::Or,
                        Box::new(right),
                    );
                }
                _ => {}
            }
        }

        Ok(result)
    }

    fn additive_expression(&mut self) -> Result<Node, ParseError> {
        let mut result = self.multiplicative_expression()?;
        println!("expression got term: {:?}", result);

        let mut token = self.get_current_token()?;
        println!("token: {:?}", token);
        let mut token_kind = token.kind().clone();
        println!("token_kind: {:?}", token_kind);

        while token_kind == TokenKind::Plus || token_kind == TokenKind::Minus {
            let line = token.line().clone();
            let column = token.column().clone();

            match token_kind {
                TokenKind::Plus => {
                    self.eat(TokenKind::Plus)?;
                    let right = self.multiplicative_expression()?;
                    println!("plus.right: {:?}", right);

                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::Plus,
                        Box::new(right),
                    );
                }
                TokenKind::Minus => {
                    self.eat(TokenKind::Minus)?;
                    let right = self.multiplicative_expression()?;
                    println!("minus.right: {:?}", right);

                    result = Node::BinaryExpression(
                        Box::new(result),
                        BinaryOperator::Minus,
                        Box::new(right),
                    );
                }
                _ => bail!(ParseError::UnexpectedToken(token_kind, line, column)),
            }
            token = self.get_current_token()?;
            println!("new token: {:?}", token);
            token_kind = token.kind().clone();
            println!("new token_kind: {:?}", token_kind);
        }

        Ok(result)
    }

    fn multiplicative_expression(&mut self) -> Result<Node, ParseError> {
        let mut left = self.factor()?;

        let mut current_token = self.get_current_token()?;
        let mut token_kind = current_token.kind().clone();

        while token_kind == TokenKind::Multiply
            || token_kind == TokenKind::Divide
            || token_kind == TokenKind::Modulo
        {
            let line = current_token.line().clone();
            let column = current_token.column().clone();
            println!("term.token_kind: {:?}", token_kind);

            match token_kind {
                TokenKind::Multiply => {
                    self.eat(TokenKind::Multiply)?;
                    let right = self.factor()?;

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Multiply,
                        Box::new(right),
                    );
                }
                TokenKind::Divide => {
                    println!("term.divide.eating divide");
                    self.eat(TokenKind::Divide)?;
                    let right = self.factor()?;
                    println!("term.divide.new_right: {:?}", right);

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Divide,
                        Box::new(right),
                    );
                }
                TokenKind::Modulo => {
                    self.eat(TokenKind::Modulo)?;
                    let right = self.factor()?;

                    left = Node::BinaryExpression(
                        Box::new(left),
                        BinaryOperator::Modulo,
                        Box::new(right),
                    );
                }
                _ => bail!(ParseError::UnexpectedToken(token_kind, line, column)),
            }

            current_token = self.get_current_token()?;
            token_kind = current_token.kind().clone();
        }

        Ok(left)
    }

    fn identifier(&mut self) -> Result<Node, ParseError> {
        let identifier = self.eat(TokenKind::Identifier)?;
        let identifier_clone = dyn_clone::clone_box(&**identifier);
        let token: Box<dyn Any> = identifier_clone.into_any();

        match token.downcast_ref::<Identifier>() {
            Some(identifier) => return Ok(Node::Identifier(identifier.value())),
            None => bail!(ParseError::UnexpectedToken(
                identifier.kind(),
                identifier.line(),
                identifier.column()
            )),
        }
    }

    fn unary_expression(&mut self) -> Result<Node, ParseError> {
        let token = self.get_current_token()?;

        let operator = match token.kind() {
            TokenKind::Increment => {
                self.eat(TokenKind::Increment)?;
                UnaryOperator::Increment
            }
            TokenKind::Decrement => {
                self.eat(TokenKind::Decrement)?;
                UnaryOperator::Decrement
            }
            TokenKind::Plus => {
                self.eat(TokenKind::Plus)?;
                UnaryOperator::Plus
            }
            TokenKind::Minus => {
                self.eat(TokenKind::Minus)?;
                UnaryOperator::Minus
            }
            TokenKind::Not => {
                self.eat(TokenKind::Not)?;
                UnaryOperator::Negation
            }
            kind => bail!(ParseError::UnexpectedToken(
                kind,
                token.line(),
                token.column()
            )),
        };

        let node = self.factor()?;

        Ok(Node::UnaryExpression(Box::new(node), operator))
    }

    fn factor(&mut self) -> Result<Node, ParseError> {
        let current_token = self.get_current_token()?;

        let token_kind = current_token.kind().clone();
        let line = current_token.line().clone();
        let column = current_token.column().clone();

        let curr_token_clone = dyn_clone::clone_box(&**current_token);
        let token: Box<dyn Any> = curr_token_clone.into_any();

        match token_kind {
            TokenKind::Identifier => self.identifier(),
            TokenKind::Integer => {
                if let Some(integer) = token.downcast_ref::<Integer>() {
                    self.eat(TokenKind::Integer)?;
                    return Ok(Node::IntegerLiteral(integer.value()));
                }
                bail!(ParseError::UnexpectedToken(token_kind, line, column))
            }
            TokenKind::Decimal => {
                if let Some(decimal) = token.downcast_ref::<Decimal>() {
                    self.eat(TokenKind::Decimal)?;
                    return Ok(Node::DecimalLiteral(decimal.value()));
                }
                bail!(ParseError::UnexpectedToken(token_kind, line, column))
            }
            TokenKind::StringLiteral => {
                if let Some(string_literal) = token.downcast_ref::<StringLiteral>() {
                    self.eat(TokenKind::StringLiteral)?;
                    return Ok(Node::StringLiteral(string_literal.value()));
                }
                bail!(ParseError::UnexpectedToken(token_kind, line, column))
            }
            TokenKind::OpenSquareBracket => {
                self.eat(TokenKind::OpenSquareBracket)?;

                let mut items: Vec<Box<Node>> = vec![Box::new(self.binary_expression()?)];

                while self.get_current_token()?.kind() == TokenKind::Comma {
                    self.eat(TokenKind::Comma)?;
                    let item = Box::new(self.binary_expression()?);
                    items.push(item);
                }

                self.eat(TokenKind::CloseSquareBracket)?;

                Ok(Node::ArrayExpression(items))
            }
            TokenKind::True | TokenKind::False => {
                self.eat(token_kind)?;

                Ok(Node::BoolLiteral(token_kind == TokenKind::True))
            }
            TokenKind::OpenParen => {
                self.eat(TokenKind::OpenParen)?; // eat open paren
                let expr = self.binary_expression()?;
                self.eat(TokenKind::CloseParen)?; // eat close paren
                return Ok(expr);
            }
            _ => self.unary_expression(),
        }
    }

    // fn factor(&mut self) -> Result<Node, ParseError> {
    //     let current_token = self.get_current_token()?;
    //     let token = current_token as &dyn Any;

    //     if let Some(identifier) = token.downcast_ref::<Identifier>() {
    //         self.eat(TokenKind::Identifier);
    //         return Ok(Node::Identifier(identifier.value()));
    //     }

    //     if let Some(integer) = token.downcast_ref::<Integer>() {
    //         self.eat(TokenKind::Integer);
    //         return Ok(Node::IntegerLiteral(integer.value()));
    //     }

    //     if let Some(decimal) = token.downcast_ref::<Decimal>() {
    //         self.eat(TokenKind::Decimal);
    //         return Ok(Node::DecimalLiteral(decimal.value()));
    //     }

    //     match current_token.kind() {
    //         TokenKind::OpenParen => {
    //             self.eat(TokenKind::OpenParen); // eat open paren
    //             let expr = self.expression()?;
    //             self.eat(TokenKind::CloseParen); // eat close paren
    //             Ok(expr)
    //         }
    //         _ => bail!(ParseError::UnexpectedToken(
    //             current_token.kind(),
    //             current_token.line(),
    //             current_token.column()
    //         )),
    //     }
    // }
}
