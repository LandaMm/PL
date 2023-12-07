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
    StringLiteral(String),                                  // value
    BinaryExpression(Box<Node>, BinaryOperator, Box<Node>), // left, operator, right
    VariableDeclaration(String, Option<Box<Node>>, bool),   // var_name, value, is_constant
    ArrayExpression(Vec<Box<Node>>),                        // array_items
}
