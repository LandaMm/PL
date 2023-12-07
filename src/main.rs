use std::fs;

use lexer::Lexer;

use crate::{
    lexer::token::{Token, TokenKind},
    parser::ast::Parser,
};

// use crate::lexer::token::TokenKind;

mod lexer;
mod macros;
mod parser;

fn read_file(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}

fn main() {
    let source = read_file("test/test-variable-declaration.pl".to_string());
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => {
            println!("tokens: {:#?}", lexer.tokens);
            let with_no_newline: Vec<Box<dyn Token>> = lexer
                .tokens
                .into_iter()
                .filter(|tok| !matches!(tok.kind(), TokenKind::Newline))
                .collect();
            let mut parser = Parser::new(with_no_newline);
            match parser.produce_ast() {
                Ok(program) => println!("ast: {:#?}", program),
                Err(err) => panic!("Error while parsing: {}", err),
            }
        }
    }
}
