use lexer::Lexer;

mod lexer;
mod macros;

fn main() {
    let source = "
# functions
def mul(a, b) {
  x = a * b
  return x
}

mul(5, 10)
    ";
    let mut lexer = Lexer::new(source.to_string());
    match lexer.tokenize() {
        Err(err) => panic!("Error: {}", err),
        Ok(_) => println!("tokens: {:#?}", lexer.tokens),
    }
}
