use std::{collections::HashMap, env::args, fs::read_to_string};

use lexer::{Lexer, MathToken, Token};
use parser::{ASTNode, Parser};

mod lexer;
mod parser;

fn get_path() -> Option<String> {
    let run_args: Vec<String> = args().collect();
    if run_args.len() > 1 {
        Some(run_args[1].clone())
    } else {
        None
    }
}

#[derive(Debug)]
pub enum Memory {
    Number(f64, bool),
    String(String, bool),
}

pub fn binary_op(bop: ASTNode) -> f64 {
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
            } else {
                panic!("CRY.ERROR: Invalid right expression in binary expression");
            }
        } else if let ASTNode::Number(n) = *right {
            match op {
                Token::Arithmetic(MathToken::Plus) => binary_op(*left) + n,
                Token::Arithmetic(MathToken::Minus) => binary_op(*left) - n,
                Token::Arithmetic(MathToken::Divide) => binary_op(*left) / n,
                Token::Arithmetic(MathToken::Multiply) => binary_op(*left) * n,
                _ => panic!("CRY.ERROR: Invalid binary operation"),
            }
        } else {
            panic!("CRY.ERROR: Invalid type detected in binary operation");
        }
    } else {
        panic!("CRY.ERROR: Invalid binary operation");
    }
}

pub fn ast_to_memory(node: ASTNode, is_mut: bool) -> Memory {
    match node {
        ASTNode::String(s) => Memory::String(s, is_mut),
        ASTNode::Number(n) => Memory::Number(n, is_mut),
        ASTNode::BinaryOp { left, op, right } => {
            Memory::Number(binary_op(ASTNode::BinaryOp { left, op, right }), is_mut)
        }
        _ => Memory::String("This Memory is invalid.".to_string(), is_mut),
    }
}

fn main() {
    let args_path: Option<String> = get_path();
    let file_path = if let Some(fp) = args_path {
        fp
    } else {
        "./main.cry".to_string()
    };
    let file = read_to_string(file_path).unwrap();
    let mut lexer = Lexer::new(file);
    let mut tokens = Vec::new();
    while let token = lexer.next_token() {
        if token == Token::EOF {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    let mut virtual_brain: HashMap<String, Box<Memory>> = HashMap::new();

    if let ASTNode::Program(nodes) = &ast {
        for node in nodes {
            match node {
                ASTNode::Let(ident, val) => {
                    println!("{ident} is being mutably assigned to {val:?}.");
                    virtual_brain.insert(
                        ident.to_string(),
                        Box::new(ast_to_memory(*val.clone(), true)),
                    );
                }
                ASTNode::Final(ident, val) => {
                    virtual_brain.insert(
                        ident.to_string(),
                        Box::new(ast_to_memory(*val.clone(), false)),
                    );
                }
                _ => {}
            }
        }
    }

    println!("{ast:#?}");
    println!("{virtual_brain:?}");
}
