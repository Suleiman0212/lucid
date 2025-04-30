#![allow(unused)]

use crate::lexer::Token;
use core::panic;

#[derive(Debug)]
pub enum Expr {
    StackDecl {
        name: String,
        typ: String,
        value: Box<Expr>,
    },
    Addition {
        left: Box<Expr>,
        right: Box<Expr>,
    },
    StringLiteral(String),
    NumberLiteral(i64),
    Identifier(String),
    Output(Box<Expr>),
    Input(Box<Expr>),
}

fn parse_literal(token: &Token) -> Option<Expr> {
    match token {
        Token::StringLiteral(s) => Some(Expr::StringLiteral(s.clone())),
        Token::NumberLiteral(n) => Some(Expr::NumberLiteral(*n)),
        _ => None,
    }
}

fn parse_addition_expr(tokens: &[Token], i: &mut usize) -> Option<Expr> {
    let mut left = parse_basic_expr(tokens, i)?;

    while *i < tokens.len() {
        if let Token::Add = tokens[*i] {
            *i += 1;
            let right = parse_basic_expr(tokens, i)?;
            left = Expr::Addition {
                left: Box::new(left),
                right: Box::new(right),
            };
        } else {
            break;
        }
    }

    Some(left)
}

fn parse_basic_expr(tokens: &[Token], i: &mut usize) -> Option<Expr> {
    if *i >= tokens.len() {
        return None;
    }

    let expr = match &tokens[*i] {
        Token::Identifier(name) => {
            *i += 1;
            Expr::Identifier(name.clone())
        }
        Token::StringLiteral(val) => {
            *i += 1;
            Expr::StringLiteral(val.clone())
        }
        Token::NumberLiteral(n) => {
            *i += 1;
            Expr::NumberLiteral(*n)
        }
        _ => return None,
    };

    Some(expr)
}

fn parse_stack_decl(tokens: &[Token], i: &mut usize) -> Option<Expr> {
    if *i >= tokens.len() || !matches!(tokens.get(*i), Some(Token::Stack)) {
        return None;
    }

    if tokens.len() > *i + 5 {
        if let (
            Token::Stack,
            Token::Identifier(name),
            Token::Colon,
            Token::Type(typ),
            Token::IntoStream,
            value_token,
        ) = (
            &tokens[*i],
            &tokens[*i + 1],
            &tokens[*i + 2],
            &tokens[*i + 3],
            &tokens[*i + 4],
            &tokens[*i + 5],
        ) {
            if let Some(value_expr) = parse_literal(value_token) {
                *i += 6;
                return Some(Expr::StackDecl {
                    name: name.clone(),
                    typ: typ.clone(),
                    value: Box::new(value_expr),
                });
            } else {
                panic!("Expected literal after <-");
            }
        }
    }

    if tokens.len() > *i + 3 {
        if let (Token::Stack, Token::Identifier(name), Token::Colon, Token::Type(typ)) = (
            &tokens[*i],
            &tokens[*i + 1],
            &tokens[*i + 2],
            &tokens[*i + 3],
        ) {
            *i += 4;
            let default_value = match typ.as_str() {
                "text" => Expr::StringLiteral("".into()),
                "num" => Expr::NumberLiteral(0),
                _ => panic!("Unsupported type: {}", typ),
            };
            return Some(Expr::StackDecl {
                name: name.clone(),
                typ: typ.clone(),
                value: Box::new(default_value),
            });
        }
    }

    panic!("Invalid stack declaration");
}

fn parse_output(tokens: &[Token], i: &mut usize) -> Option<Expr> {
    if *i + 2 >= tokens.len() {
        return None;
    }

    // Проверка начала: out <-
    if !matches!(tokens.get(*i), Some(Token::Out))
        || !matches!(tokens.get(*i + 1), Some(Token::IntoStream))
    {
        return None;
    }

    *i += 2;

    let mut expr = match tokens.get(*i)? {
        Token::Identifier(name) => Expr::Identifier(name.clone()),
        Token::StringLiteral(val) => Expr::StringLiteral(val.clone()),
        Token::NumberLiteral(val) => Expr::NumberLiteral(*val),
        _ => panic!("Invalid value after out <-"),
    };

    *i += 1;

    // Пока есть `+`, продолжаем строить Expr::Addition
    while *i < tokens.len() {
        if !matches!(tokens.get(*i), Some(Token::Add)) {
            break;
        }

        *i += 1;

        let rhs = match tokens.get(*i)? {
            Token::Identifier(name) => Expr::Identifier(name.clone()),
            Token::StringLiteral(val) => Expr::StringLiteral(val.clone()),
            Token::NumberLiteral(val) => Expr::NumberLiteral(*val),
            _ => panic!("Invalid right-hand side in addition"),
        };

        *i += 1;

        expr = Expr::Addition {
            left: Box::new(expr),
            right: Box::new(rhs),
        };
    }

    Some(Expr::Output(Box::new(expr)))
}

fn parse_input(tokens: &[Token], i: &mut usize) -> Option<Expr> {
    if *i + 2 >= tokens.len() {
        return None;
    }

    if !matches!(tokens.get(*i), Some(Token::In))
        || !matches!(tokens.get(*i + 1), Some(Token::FromStream))
    {
        return None;
    }

    if let Token::Identifier(name) = &tokens[*i + 2] {
        *i += 3;
        Some(Expr::Input(Box::new(Expr::Identifier(name.clone()))))
    } else {
        panic!("Expected identifier after in ->");
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    let mut exprs = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        if let Some(expr) = parse_stack_decl(&tokens, &mut i) {
            exprs.push(expr);
        } else if let Some(expr) = parse_output(&tokens, &mut i) {
            exprs.push(expr);
        } else if let Some(expr) = parse_input(&tokens, &mut i) {
            exprs.push(expr);
        } else {
            panic!("Unknown token: {:?}", tokens[i]);
        }
    }

    exprs
}
