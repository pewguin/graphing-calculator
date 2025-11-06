use std::{collections::VecDeque, fmt::Display, ops::RangeBounds};

use crate::parsing::ast::*;

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Constant(f32),
    Variable(u32),
    Operator(Operator),
    LeftGrouping,
    RightGrouping
}

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Unary(UnaryOperator),
    Binary(BinaryOperator)
}
impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Unary(op) => op.precedence(),
            Operator::Binary(op) => op.precedence(),
        }
    }
}


pub fn tokenize(string: &String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut num_str = String::new();
    let mut str_iter = string.chars().into_iter();

    while let Some(c) = str_iter.next() {
        if ('0'..'9').contains(&c) || c == '.'{
            num_str.push(c);
            continue;
        } else if num_str.len() > 0 {
            tokens.push(Token::Constant(num_str.parse().expect("Structured number incorrectly.")));
            num_str = String::new();
        }
        match c {
            'x' => tokens.push(Token::Variable(0)),
            '+' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Addition))),
            '-' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Subtraction))),
            '*' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Multiplication))),
            '/' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Division))),
            '%' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Modulo))),
            '^' => tokens.push(Token::Operator(Operator::Binary(BinaryOperator::Exponentiation))),
            '(' => tokens.push(Token::LeftGrouping),
            ')' => tokens.push(Token::RightGrouping),
            _ => {}
        }
    }
    if num_str.len() > 0 {
        tokens.push(Token::Constant(num_str.parse().expect("Structured number incorrectly.")));
    }

    tokens
}

pub fn parse(tokens: Vec<Token>) -> Result<Box<Value>, String> {
    let mut opers_stack: VecDeque<Token> = VecDeque::new();
    let mut output_stack: VecDeque<Value> = VecDeque::new();

    fn process_operator(opers_stack: &mut VecDeque<Token>, output_stack: &mut VecDeque<Value>) -> Result<(), String> {
        let rhs = match output_stack.pop_front() {
            Some(rhs) => rhs,
            None => return Err(String::from("Too few operands")),
        };
        let lhs = match output_stack.pop_front() {
            Some(lhs) => lhs,
            None => return Err(String::from("Too few operands")),
        };
        let new_val = match opers_stack.pop_front() {
            Some(Token::Operator(Operator::Binary(bo))) => Value::BinaryOperator {
                operator: bo,
                operands: (
                    Box::new(lhs),
                    Box::new(rhs)
                )
            },
            Some(token) => return Err(String::from("Invalid operator in operator stack")),
            None => return Err(String::from("Too few operators")),
        };
        output_stack.push_front(new_val);
        Ok(())
    }

    for token in tokens {
        match token {
            Token::Constant(c) => output_stack.push_front(Value::Constant(c)),
            Token::Variable(i) => output_stack.push_front(Value::Variable(i)),
            Token::Operator(op) => {
                while !opers_stack.is_empty() &&
                    !matches!(opers_stack.front().unwrap(), Token::LeftGrouping) &&
                    matches!(opers_stack.front().unwrap(), Token::Operator(o) if o.precedence() >= op.precedence()) {
                    if let Err(e) = process_operator(&mut opers_stack, &mut output_stack) {
                        return Err(e);
                    }
                }
                opers_stack.push_front(token);
            }
            Token::LeftGrouping => opers_stack.push_front(token),
            Token::RightGrouping => {
                while !matches!(match opers_stack.front() {
                    Some(token) => token,
                    None => return Err(String::from("Malformed parenthesis")),
                }, Token::LeftGrouping) {
                    if let Err(e) = process_operator(&mut opers_stack, &mut output_stack) {
                        return Err(e);
                    }
                }
                opers_stack.pop_front();
            }
        }
    }
    while !opers_stack.is_empty() {
        if let Err(e) = process_operator(&mut opers_stack, &mut output_stack) {
            return Err(e);
        }
    }
    match output_stack.pop_front() {
        Some(val) => Ok(Box::new(val)),
        None => Err(String::from("Empty expression")),
    }
}