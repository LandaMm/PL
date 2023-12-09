use std::{cmp::Ordering, env, fs};

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
    let args: Vec<String> = env::args().collect();

    let filename = match args.len().cmp(&1) {
        Ordering::Equal | Ordering::Less => "test/test-assignment-expression.pl",
        Ordering::Greater => args.get(1).unwrap(),
    };

    let source = read_file(filename.to_string());
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => {
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
