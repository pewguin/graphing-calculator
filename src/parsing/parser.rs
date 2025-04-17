use std::{collections::VecDeque, fmt::Display, ops::RangeBounds};

use crate::parsing::ast::*;

#[derive(Clone, Copy, Debug)]
pub enum Token {
    Constant(f32),
    Variable(i32),
    Operator(OperatorToken),
    LeftGrouping,
    RightGrouping
}

#[derive(Clone, Copy, Debug)]
pub enum OperatorToken {
    Unary(UnaryOperator),
    Binary(BinaryOperator)
}

// Lexer converts strings to tokens
pub struct Lexer {
    idx: usize
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            idx: 0,
        }
    }

    pub fn tokenize(&mut self, string: String) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut num_str = "".to_string();

        while self.idx < string.chars().count() {
            let c = string.chars().nth(self.idx).unwrap();
            if ('0'..'9').contains(&c) || c == '.'{
                num_str.push(c);
                self.idx += 1;
                continue;
            }
            else if num_str.len() > 0 {
                tokens.push(Token::Constant(num_str.parse().expect("Structured number incorrectly.")));
                num_str = "".to_string();
            }
            match c {
                'x' => tokens.push(Token::Variable(0)),
                '+' => tokens.push(Token::Operator(OperatorToken::Binary(BinaryOperator::Addition))),
                '-' => tokens.push(Token::Operator(OperatorToken::Binary(BinaryOperator::Subtraction))),
                '*' => tokens.push(Token::Operator(OperatorToken::Binary(BinaryOperator::Multiplication))),
                '/' => tokens.push(Token::Operator(OperatorToken::Binary(BinaryOperator::Division))),
                '^' => tokens.push(Token::Operator(OperatorToken::Binary(BinaryOperator::Exponentiation))),
                _ => {}
            }
            self.idx += 1;
        }
        if num_str.len() > 0 {
            tokens.push(Token::Constant(num_str.parse().expect("Structured number incorrectly.")));
        }

        tokens
    }
}

// Parser converts tokens to an AST
pub struct Parser {}

impl Parser {
    pub fn parse(tokens_infix: Vec<Token>) -> Option<Box<Node>> {
        let mut opers_stack: VecDeque<Token> = VecDeque::new();
        let mut postfix: Vec<Token> = Vec::new();
        
        for token in tokens_infix {
            match token {
                Token::Constant(_) => postfix.push(token),
                Token::Variable(_) => postfix.push(token),
                Token::LeftGrouping => opers_stack.push_front(token),
                Token::RightGrouping => {
                    while match opers_stack.front().expect("parenthesis error") {
                        Token::LeftGrouping => true,
                        _ => false,
                    }
                    {
                        postfix.push(opers_stack.pop_front().unwrap());
                    }
                    opers_stack.pop_front();
                }
                Token::Operator(OperatorToken::Binary(oper)) => {
                    while !opers_stack.is_empty() && oper.precedence() <= match opers_stack.front().unwrap() {
                        Token::Operator(OperatorToken::Binary(oper)) => oper.precedence(),
                        _ => panic!(),
                    } {
                        postfix.push(opers_stack.pop_front().unwrap());
                    }
                    opers_stack.push_front(token);
                },
                _ => panic!("not implemented")
            }
        }

        while opers_stack.front().is_some() {
            postfix.push(opers_stack.pop_front().unwrap());
        }

        let mut leaf_stack: VecDeque<Node> = VecDeque::new();

        for token in postfix {
            match token {
                Token::Constant(f) => leaf_stack.push_front(Node::Constant(f)),
                Token::Variable(i) => leaf_stack.push_front(Node::Variable(i)),
                Token::Operator(OperatorToken::Binary(oper)) => {
                    let right = match leaf_stack.pop_front() {
                        Some(node) => node,
                        None => return None
                    };
                    let left = match leaf_stack.pop_front() {
                        Some(node) => node,
                        None => return None
                    };
                    leaf_stack.push_front(
                        Node::BinaryOperator { 
                            operator: oper, 
                            operands: 
                                (Box::new(left), 
                                Box::new(right)) })
                }
                _ => panic!("kys")
            }
        }
        if leaf_stack.len() == 1 {
            Some(Box::new(leaf_stack.front().unwrap().clone()))
        } else {
            None
        }
    }
}