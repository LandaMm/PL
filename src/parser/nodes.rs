#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Increment,
    Decrement,
    Plus,
    Minus,
    Negation,
}

#[derive(Debug, Clone)]
pub enum AssignmentOperator {
    Equals,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulation,
}

#[derive(Debug, Clone)]
pub enum Node {
    // statements
    Program(Vec<Box<Node>>),                                        // body[]
    VariableDeclaration(String, Option<Box<Node>>, bool),           // var_name, value, is_constant
    BlockStatement(Vec<Box<Node>>),                                 // body[]
    FunctionDeclaration(Box<Node>, Vec<Box<Node>>, Box<Node>), // identifier, params, block_statement(body)
    IfStatement(Box<Node>, Box<Node>, Option<Box<Node>>), // condition, body (consequent), alternate
    ForInStatement(Box<Node>, Box<Node>, Box<Node>),      // left, right, body[] (block_statement)
    ReturnStatement(Box<Node>),                           // value
    ImportStatement(Box<Node>),                           // import entity
    ClassDeclaration(Box<Node>, Option<Box<Node>>, Vec<Box<Node>>), // id, super_class, body
    PropertyDefinition(Box<Node>, Box<Node>, bool),       // id, value, is_static
    MethodDefinition(Box<Node>, Vec<Box<Node>>, Box<Node>, bool), // key, params, body, is_static

    // literals
    IntegerLiteral(usize), // value
    DecimalLiteral(f64),   // value
    Identifier(String),    // value
    StringLiteral(String), // value
    BoolLiteral(bool),     // value
    NullLiteral(),         // nothing, cz it's null

    // expressions
    BinaryExpression(Box<Node>, BinaryOperator, Box<Node>), // left, operator, right
    ArrayExpression(Vec<Box<Node>>),                        // array_items
    LogicalExpression(Box<Node>, LogicalOperator, Box<Node>), // left, operator, right
    UnaryExpression(Box<Node>, UnaryOperator),              // unary target node, unary operator
    MemberExpression(Box<Node>, Box<Node>, bool),           // object, property, computed
    CallExpression(Box<Node>, Vec<Box<Node>>),              // callee, arguments
    AssignmentExpression(Box<Node>, AssignmentOperator, Box<Node>), // assigne, operator, value
}
