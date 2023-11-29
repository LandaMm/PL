#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Box<Node>>),                                // body[]
    IntegerLiteral(usize),                                  // value
    DecimalLiteral(f64),                                    // value
    Identifier(String),                                     // value
    BinaryExpression(Box<Node>, BinaryOperator, Box<Node>), // left, operator, right
}
