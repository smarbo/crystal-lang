use core::panic;

use super::lexer::Token;

// ASTNode Enum
#[derive(Debug, PartialEq, Clone)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Let(String, Box<ASTNode>),
    Final(String, Box<ASTNode>),
    Number(f64),
    Identifier(String),
    String(String),
    FunCall(String, Box<ASTNode>),
    BinaryOp {
        left: Box<ASTNode>,
        op: Token,
        right: Box<ASTNode>,
    },
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }

    pub fn current_token(&self) -> &Token {
        self.tokens.get(self.position).unwrap_or(&Token::EOF)
    }

    pub fn advance(&mut self) {
        self.position += 1;
    }

    pub fn parse(&mut self) -> ASTNode {
        let mut program = Vec::new();
        while self.current_token() != &Token::EOF {
            program.push(self.statement());
        }

        ASTNode::Program(program)
    }

    pub fn statement(&mut self) -> ASTNode {
        match self.current_token() {
            Token::Let => self.let_statement(),
            Token::Final => self.final_statement(),
            _ => self.expression(),
        }
    }

    pub fn let_statement(&mut self) -> ASTNode {
        self.advance();
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            if *self.current_token() != Token::Equals {
                panic!("CRY.ERROR: Expected '=' after identifier");
            }
            self.advance();
            let value = self.expression();
            if *self.current_token() != Token::Semicolon {
                panic!("CRY.ERROR: Expected ';' after expression");
            }
            self.advance();
            ASTNode::Let(name, Box::new(value))
        } else {
            panic!("CRY.ERROR: Expected identifier after 'let'");
        }
    }

    pub fn final_statement(&mut self) -> ASTNode {
        self.advance();
        if let Token::Identifier(name) = self.current_token().clone() {
            self.advance();
            if *self.current_token() != Token::Equals {
                panic!("CRY.ERROR: Expected '=' after identifier");
            }
            self.advance();
            let value = self.expression();
            if *self.current_token() != Token::Semicolon {
                panic!("CRY.ERROR: Expected ';' after expression");
            }
            self.advance();
            ASTNode::Final(name, Box::new(value))
        } else {
            panic!("CRY.ERROR: Expected identifier after 'final'");
        }
    }

    pub fn expression(&mut self) -> ASTNode {
        let mut left = self.term();
        while matches!(self.current_token(), Token::Arithmetic(..)) {
            let op = self.current_token().clone();
            self.advance();
            let right = self.term();
            left = ASTNode::BinaryOp {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }
        left
    }

    pub fn term(&mut self) -> ASTNode {
        match self.current_token() {
            Token::Number(n) => {
                let number = *n;
                self.advance();
                ASTNode::Number(number)
            },
            Token::Identifier(i) => {
                let ident = i.clone();
                self.advance();
                ASTNode::Identifier(ident)
            },
            Token::String(v) => {
                let value = v.clone();
                self.advance();
                ASTNode::String(value)
            },
            _ => panic!("CRY.ERROR: Unexpected token: {:?}", self.current_token()),
        }
    }
}
