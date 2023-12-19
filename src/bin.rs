use std::fs;

use pl_ast::{token::TokenKind, Lexer, Parser};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let filename = if args.len() > 1 {
        args.get(1).unwrap()
    } else {
        "test/main.amr"
    };
    let source =
        fs::read_to_string(filename).expect(format!("Failed to read file '{}'", filename).as_str());
    let mut tokenizer = Lexer::new(source);
    tokenizer.tokenize().expect("Failed to tokenize");
    println!("{:#?}\n-------------------------\n\n", tokenizer.tokens);

    let mut parser = Parser::new(
        tokenizer
            .tokens
            .into_iter()
            .filter(|t| t.kind() != TokenKind::Newline)
            .collect(),
    );
    let ast = parser.produce_ast().expect("Failed to parse tokens");
    println!("{:#?}\n", ast)
}
