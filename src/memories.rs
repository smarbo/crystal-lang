use std::collections::HashMap;

use super::{
    lexer::{MathToken, Token},
    parser::ASTNode,
};
#[derive(Debug)]
pub enum Memory {
    Number(f64, bool),
    String(String, bool),
}

pub type Context = HashMap<String, Box<Memory>>;

pub fn binary_op(bop: ASTNode, context: &Context) -> f64 {
    println!("Executing binary operation.");
    if let ASTNode::BinaryOp { left, op, right } = bop {
        if let ASTNode::Number(x) = *left {
            if let ASTNode::Number(y) = *right {
                match op {
                    Token::Arithmetic(MathToken::Plus) => x + y,
                    Token::Arithmetic(MathToken::Minus) => x - y,
                    Token::Arithmetic(MathToken::Divide) => x / y,
                    Token::Arithmetic(MathToken::Multiply) => x * y,
                    _ => panic!("CRY.ERROR: Invalid binary operation"),
                }
            } else if let ASTNode::Identifier(yi) = *right {
                if let Memory::Number(v, _) = **context.get(&yi).unwrap() {
                    match op {
                        Token::Arithmetic(MathToken::Plus) => x + v,
                        Token::Arithmetic(MathToken::Minus) => x - v,
                        Token::Arithmetic(MathToken::Divide) => x / v,
                        Token::Arithmetic(MathToken::Multiply) => x * v,
                        _ => panic!("CRY.ERROR: Invalid binary operation"),
                    }
                } else {
                    panic!("Identifier {yi} is not found in context.");
                }
            } else {
                panic!("CRY.ERROR: Invalid right expression in binary expression");
            }
        } else if let ASTNode::Identifier(xi) = *left {
            if let Memory::Number(v, _) = **context.get(&xi).unwrap() {
                if let ASTNode::Number(y) = *right {
                    match op {
                        Token::Arithmetic(MathToken::Plus) => v + y,
                        Token::Arithmetic(MathToken::Minus) => v - y,
                        Token::Arithmetic(MathToken::Divide) => v / y,
                        Token::Arithmetic(MathToken::Multiply) => v * y,
                        _ => panic!("CRY.ERROR: Invalid binary operation"),
                    }
                } else if let ASTNode::Identifier(yi) = *right {
                    if let Memory::Number(yv, _) = **context.get(&yi).unwrap() {
                        match op {
                            Token::Arithmetic(MathToken::Plus) => v + yv,
                            Token::Arithmetic(MathToken::Minus) => v - yv,
                            Token::Arithmetic(MathToken::Divide) => v / yv,
                            Token::Arithmetic(MathToken::Multiply) => v * yv,
                            _ => panic!("CRY.ERROR: Invalid binary operation"),
                        }
                    } else {
                        panic!("CRY.ERROR: Invalid binary operation");
                    }
                } else {
                    panic!("CRY.ERROR: Invalid binary operation");
                }
            } else {
                panic!("CRY.ERROR: Invalid binary operation");
            }
        } else if let ASTNode::Number(n) = *right {
            match op {
                Token::Arithmetic(MathToken::Plus) => binary_op(*left, context) + n,
                Token::Arithmetic(MathToken::Minus) => binary_op(*left, context) - n,
                Token::Arithmetic(MathToken::Divide) => binary_op(*left, context) / n,
                Token::Arithmetic(MathToken::Multiply) => binary_op(*left, context) * n,
                _ => panic!("CRY.ERROR: Invalid binary operation"),
            }
        } else if let ASTNode::Identifier(ri) = *right {
            if let Memory::Number(rv, _) = **context.get(&ri).unwrap() {
                match op {
                    Token::Arithmetic(MathToken::Plus) => binary_op(*left, context) + rv,
                    Token::Arithmetic(MathToken::Minus) => binary_op(*left, context) - rv,
                    Token::Arithmetic(MathToken::Divide) => binary_op(*left, context) / rv,
                    Token::Arithmetic(MathToken::Multiply) => binary_op(*left, context) * rv,
                    _ => panic!("CRY.ERROR: Invalid binary operation"),
                }
            } else {
                panic!("CRY.ERROR: Invalid right side binary expression.");
            }
        } else {
            println!("{left:?} ++++++ {op:?} +++++++++ {right:?}");
            panic!("CRY.ERROR: Invalid type detected in binary operation");
        }
    } else if let ASTNode::Number(n) = bop {
        n
    } else if let ASTNode::Identifier(ident) = bop {
        // TODO Add ident compound assignment
        1f64
    } else {
        panic!("CRY.ERROR: Invalid binary operation");
    }
}

pub fn ast_to_memory(node: ASTNode, is_mut: bool, context: &Context) -> Memory {
    match node {
        ASTNode::String(s) => Memory::String(s, is_mut),
        ASTNode::Number(n) => Memory::Number(n, is_mut),
        ASTNode::BinaryOp { left, op, right } => Memory::Number(
            binary_op(ASTNode::BinaryOp { left, op, right }, context),
            is_mut,
        ),
        _ => Memory::String("This Memory is invalid.".to_string(), is_mut),
    }
}
