use lexer::Lexer;

mod lexer;
mod macros;
mod parser;

fn main() {
    let source = "
if x == a and a != 0 {
  y = x
} else if a > 0 or a == -5 {
  y = -x
} else {
  y = 0
  y += a
}
    ";
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => println!("tokens: {:#?}", lexer.tokens),
    }
}
