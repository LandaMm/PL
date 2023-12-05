use lexer::Lexer;

// use crate::lexer::token::TokenKind;

mod lexer;
mod macros;
// mod parser;

fn main() {
    let source = "
(fooBar - 10) * 2 / 5 % 10
    ";
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => {
            println!("tokens: {:#?}", lexer.tokens);
            // let mut parser = Parser::new(
            //     lexer
            //         .tokens
            //         .into_iter()
            //         .filter(|tok| !matches!(tok, TokenKind::Newline))
            //         .collect(),
            // );
            // match parser.produce_ast() {
            //     Ok(program) => println!("ast: {:#?}", program),
            //     Err(err) => panic!("Error while parsing: {}", err),
            // }
        }
    }
}
