use std::fs;

use pl_ast::Lexer;

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
    println!("{:#?}", tokenizer.tokens);
}
