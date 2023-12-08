#[derive(Debug)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    LessThan,
    GreaterThan,
    IsEquals,
    NotEquals,
}

#[derive(Debug)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug)]
pub enum Node {
    Program(Vec<Box<Node>>),                                   // body[]
    IntegerLiteral(usize),                                     // value
    DecimalLiteral(f64),                                       // value
    Identifier(String),                                        // value
    StringLiteral(String),                                     // value
    BinaryExpression(Box<Node>, BinaryOperator, Box<Node>),    // left, operator, right
    VariableDeclaration(String, Option<Box<Node>>, bool),      // var_name, value, is_constant
    ArrayExpression(Vec<Box<Node>>),                           // array_items
    BoolLiteral(bool),                                         // value
    BlockStatement(Vec<Box<Node>>),                            // body[]
    FunctionDeclaration(Box<Node>, Vec<Box<Node>>, Box<Node>), // identifier, params, block_statement(body)
    LogicalExpression(Box<Node>, LogicalOperator, Box<Node>),  // left, operator, right
    IfStatement(Box<Node>, Box<Node>, Option<Box<Node>>), // condition, body (consequent), alternate
}
