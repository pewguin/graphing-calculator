pub enum Operator {
    Plus,
    Minus
}

pub enum Node {
    Constant(f32),
    Variable(i32),

    UnaryOperator {
        operator: Operator,
        operand: Box<Node>,
    },
    BinaryOperator {
        operator: Operator,
        operand_0: Box<Node>,
        operand_1: Box<Node>,
    }
}