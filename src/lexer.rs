#[derive(Debug, PartialEq, Clone)]
pub enum MathToken {
    Plus,
    Minus,
    Divide,
    Multiply,
    PlusEq,
    MinusEq,
    MultiplyEq,
    DivideEq,
}

// Token Enum
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    Equals,
    String(String),
    LParen,
    RParen,
    Semicolon,
    EOF,
    Arithmetic(MathToken),
    Let,
    Final,
}

// Lexer struct
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

// Implementation for Crystal Lexer with all lexing functions
impl Lexer {
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer {
            input: input.chars().collect(),
            position: 0,
            current_char: None,
        };
        lexer.current_char = lexer.input.get(0).cloned();
        lexer
    }

    pub fn advance(&mut self) {
        self.position += 1;
        self.current_char = self.input.get(self.position).cloned();
    }

    pub fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        match self.current_char {
            Some('=') => {
                self.advance();
                Token::Equals
            }
            Some('(') => {
                self.advance();
                Token::LParen
            }
            Some(')') => {
                self.advance();
                Token::RParen
            }
            Some(';') => {
                self.advance();
                Token::Semicolon
            }
            Some('+') => {
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::Arithmetic(MathToken::PlusEq)
                } else {
                    Token::Arithmetic(MathToken::Plus)
                }
            }
            Some('-') => {
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::Arithmetic(MathToken::MinusEq)
                } else {
                    Token::Arithmetic(MathToken::Minus)
                }
            }
            Some('*') => {
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::Arithmetic(MathToken::MultiplyEq)
                } else {
                    Token::Arithmetic(MathToken::Multiply)
                }
            }
            Some('/') => {
                self.advance();
                if let Some('=') = self.current_char {
                    self.advance();
                    Token::Arithmetic(MathToken::DivideEq)
                } else {
                    Token::Arithmetic(MathToken::Divide)
                }
            }
            Some(c) if c.is_alphabetic() => self.identifier(),
            Some('"') => self.string(),
            Some(c) if c.is_digit(10) => self.number(),
            None => Token::EOF,
            _ => panic!("Unexpected character: {}", self.current_char.unwrap()),
        }
    }

    pub fn number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(c) = self.current_char {
            if c.is_digit(10) || c == '.' {
                number.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(number.parse().unwrap())
    }

    pub fn identifier(&mut self) -> Token {
        let mut ident = String::new();
        while let Some(c) = self.current_char {
            if c.is_alphabetic() || c == '_' {
                ident.push(c);
                self.advance();
            } else {
                break;
            }
        }
        match ident.as_str() {
            "let" => Token::Let,
            "final" => Token::Final,
            _ => Token::Identifier(ident),
        }
    }

    pub fn string(&mut self) -> Token {
        let mut value = String::new();
        self.advance();

        while let Some(c) = self.current_char {
            if c == '"' {
                self.advance();
                break;
            } else {
                value.push(c);
                self.advance();
            }
        }

        Token::String(value)
    }
}
