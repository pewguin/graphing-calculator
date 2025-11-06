use std::fmt::{Display, Write};
use std::fmt;

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Negation,
    AbsoluteValue,
    Sin,
    Factorial
}

impl UnaryOperator {
    pub fn precedence(&self) -> u8 {
        u8::MAX
    }
}

impl UnaryOperator {
    pub fn evaluate(&self, val: f32) -> f32 {
        match self {
            UnaryOperator::Negation => -val,
            UnaryOperator::AbsoluteValue => val.abs(),
            UnaryOperator::Sin => val.sin(),
            UnaryOperator::Factorial => (1..=val.round() as i32).product::<i32>() as f32
        }
    }
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
    pub fn evaluate(&self, lhs: f32, rhs: f32) -> f32 {
        match self {
            BinaryOperator::Addition => lhs + rhs,
            BinaryOperator::Subtraction => lhs - rhs,
            BinaryOperator::Multiplication => lhs * rhs,
            BinaryOperator::Division => lhs / rhs,
            BinaryOperator::Exponentiation => lhs.powf(rhs),
            BinaryOperator::Modulo => lhs % rhs,
        }
    }
}
#[derive(Debug, Clone)]
pub enum Value {
    Constant(f32),
    Variable(u32),

    UnaryOperator {
        operator: UnaryOperator,
        operand: Box<Value>,
    },
    BinaryOperator {
        operator: BinaryOperator,
        operands: (
            Box<Value>,
            Box<Value>
        )
    }
}

impl Value {
    pub fn evaluate(&self, variables: &Vec<f32>) -> f32 {
        match self {
            Value::Constant(value) => *value,
            Value::Variable(value) => variables[*value as usize],
            Value::UnaryOperator { operator, operand } => operator.evaluate(operand.evaluate(variables)),
            Value::BinaryOperator { operator, operands } => operator.evaluate(operands.0.evaluate(variables), operands.1.evaluate(variables))
        }
    }
    fn fmt_with_indent(&self, st: &mut String, indent: usize) -> fmt::Result {
        let pad = "  ".repeat(indent);

        match self {
            Value::Constant(c) => writeln!(st, "{}{}", pad, c)?,
            Value::Variable(i) => writeln!(st, "{}{}", pad, i)?,
            Value::UnaryOperator { operator, operand } => {
                writeln!(st, "{}{:?}", pad, operator)?;
                operand.fmt_with_indent(st, indent + 1)?;
            },
            Value::BinaryOperator { operator, operands } => {
                writeln!(st, "{}{:?}", pad, operator)?;
                operands.0.fmt_with_indent(st, indent + 1)?;
                operands.1.fmt_with_indent(st, indent + 1)?;
            }
        }
        Ok(())
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        self.fmt_with_indent(&mut s, 0);
        write!(f, "{}", s)
    }
}