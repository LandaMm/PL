use lexer::Lexer;

mod lexer;
mod macros;

fn main() {
    let source = "(56.23546)+/*-#=s_ome4";
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => println!("tokens: {:?}", lexer.tokens),
    }
}
