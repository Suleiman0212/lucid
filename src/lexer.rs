#![allow(unused)]

#[derive(Clone, Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

#[derive(Debug)]
pub enum Token {
    Stack,
    Out,
    In,
    Type(String),
    NumberLiteral(i64),
    StringLiteral(String),
    Identifier(String),
    Colon,
    Assign,
    Add,
    Sub,
    Mul,
    Del,
    IntoStream,
    FromStream,
    Unknown(char),
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn next_char(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let ch = self.input[self.position];
            self.position += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn peek_char(self) -> Option<char> {
        if self.position < self.input.len() {
            Some(self.input[self.position])
        } else {
            None
        }
    }

    fn lex_string(&mut self) -> Token {
        let mut token_value = String::new();
        while let Some(ch) = self.next_char() {
            if ch == '"' {
                return Token::StringLiteral(token_value);
            }
            token_value.push(ch);
        }
        Token::StringLiteral(token_value)
    }

    fn lex_number(&mut self, first: char) -> Token {
        let mut number = String::new();
        number.push(first);
        while let Some(ch) = self.clone().peek_char() {
            if ch.is_digit(10) {
                number.push(ch);
                self.next_char();
            } else {
                break;
            }
        }
        Token::NumberLiteral(number.parse().unwrap())
    }

    fn lex_identifier(&mut self, first: char) -> Token {
        let mut identifier = String::new();
        identifier.push(first);
        while let Some(ch) = self.clone().peek_char() {
            if ch.is_alphabetic() || ch == '_' {
                identifier.push(ch);
                self.next_char();
            } else {
                break;
            }
        }

        match identifier.as_str() {
            "stack" => Token::Stack,
            "out" => Token::Out,
            "in" => Token::In,
            "text" | "num" => Token::Type(identifier),
            _ => Token::Identifier(identifier),
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(ch) = self.next_char() {
            return Some(match ch {
                ' ' | '\n' | '\t' => continue,
                ':' => Token::Colon,
                '<' if self.clone().peek_char() == Some('-') => {
                    self.next_char();
                    Token::IntoStream
                }
                '-' if self.clone().peek_char() == Some('>') => {
                    self.next_char();
                    Token::FromStream
                }
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Del,
                '"' => self.lex_string(),
                _ if ch.is_alphabetic() => self.lex_identifier(ch),
                _ if ch.is_digit(10) => self.lex_number(ch),
                _ => Token::Unknown(ch),
            });
        }
        None
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }
}
