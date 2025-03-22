pub enum UnaryOperator {
    Negation,
    AbsoluteValue,
    Sin,
    Factorial
}

pub enum BinaryOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Exponentiation,
    Modulo,
}

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

pub struct Parser {}

impl Parser {
    pub fn parse_string(equation: &str) -> Box<Node> {
        let head = Node::BinaryOperator { 
            operator: BinaryOperator::Exponentiation,
            operands: (
                Box::new(
                    Node::BinaryOperator {
                        operator: BinaryOperator::Subtraction,
                        operands: (
                            Box::new(
                                Node::BinaryOperator { 
                                    operator: BinaryOperator::Modulo, 
                                    operands: (
                                        Box::new(Node::Variable(0)),
                                        Box::new(Node::Constant(5.0)),
                                    )
                                }
                            ),
                            Box::new(Node::Constant(2.5))
                        )
                    }
                ),
                Box::new(
                    Node::Constant(2.0)
                )
            ) 
        };

        Box::new(head)
    }
}