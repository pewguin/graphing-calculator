#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Negation,
    AbsoluteValue,
    Sin,
    Factorial
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulo,
}

impl BinaryOperator {
    pub fn precedence(&self) -> u8 {
        match self {
            BinaryOperator::Addition | BinaryOperator::Subtraction => 0,
            BinaryOperator::Multiplication | BinaryOperator::Division | BinaryOperator::Modulo => 1,
            BinaryOperator::Exponentiation => 2,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Node {
    Constant(f32),
    Variable(i32),

    UnaryOperator {
        operator: UnaryOperator,
        operand: Box<Node>,
    },
    BinaryOperator {
        operator: BinaryOperator,
        operands: (
            Box<Node>,
            Box<Node>
        )
    }
}