use crate::parsing::{ast::*, parser::*};

#[derive(Clone)]
pub struct Function {
    ast_head: Box<Node>,
}

impl Function {
    pub fn new(function: &str) -> Option<Self> {
        let mut lex = Lexer::new();
        let head = Parser::parse(lex.tokenize(function.to_string()));
        if (head.is_some()) {
            Some(Self {
                ast_head: head.unwrap()
            })
        } else {
            None
        }
    }

    pub fn eval(&self, x: f32)  -> f32 {
        self.eval_recursive(x, &self.ast_head)
    }
    
    fn eval_recursive(&self, x: f32, node: &Node) -> f32 {
        match node {
            Node::Constant(i) => *i,
            Node::Variable(_) => x,
            Node::UnaryOperator { operator, operand } => match operator {
                UnaryOperator::Negation => -self.eval_recursive(x, operand),
                UnaryOperator::AbsoluteValue => self.eval_recursive(x, operand).abs(),
                UnaryOperator::Factorial => panic!("Factorials are not implemented yet."),
                UnaryOperator::Sin => self.eval_recursive(x, operand).sin(),
            },
            Node::BinaryOperator { operator, operands } => match operator {
                BinaryOperator::Addition => self.eval_recursive(x, &operands.0) + self.eval_recursive(x, &operands.1),
                BinaryOperator::Subtraction => self.eval_recursive(x, &operands.0) - self.eval_recursive(x, &operands.1),
                BinaryOperator::Multiplication => self.eval_recursive(x, &operands.0) * self.eval_recursive(x, &operands.1),
                BinaryOperator::Division => self.eval_recursive(x, &operands.0) / self.eval_recursive(x, &operands.1),
                BinaryOperator::Exponentiation => self.eval_recursive(x, &operands.0).powf(self.eval_recursive(x, &operands.1)),
                BinaryOperator::Modulo => self.eval_recursive(x, &operands.0) % self.eval_recursive(x, &operands.1),
            },
        }
    }
}