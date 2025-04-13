#![allow(unused)]

use core::panic;

use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
    StackDecl {
        name: String,
        typ: String,
        value: Box<Expr>,
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

pub fn parse(tokens: Vec<Token>) -> Vec<Expr> {
    let mut exprs = Vec::new();
    let mut i = 0;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Stack => {
                if i + 5 >= tokens.len() {
                    panic!("Unexpected end of tokens");
                }

                let value_token = &tokens[i + 5];

                if let (Token::Identifier(name), Token::Colon, Token::Type(typ), Token::Assign) = (
                    &tokens[i + 1],
                    &tokens[i + 2],
                    &tokens[i + 3],
                    &tokens[i + 4],
                ) {
                    if let Some(value_expr) = parse_literal(value_token) {
                        exprs.push(Expr::StackDecl {
                            name: name.clone(),
                            typ: typ.clone(),
                            value: Box::new(value_expr),
                        });
                        i += 6;
                    } else {
                        panic!("Expected literal after <-");
                    }
                } else if let (Token::Identifier(name), Token::Colon, Token::Type(typ)) =
                    (&tokens[i + 1], &tokens[i + 2], &tokens[i + 3])
                {
                    exprs.push(Expr::StackDecl {
                        name: name.clone(),
                        typ: typ.clone(),
                        value: match typ.as_str() {
                            "text" => Box::new(Expr::StringLiteral("".into())),
                            "num" => Box::new(Expr::NumberLiteral(0)),
                            _ => panic!("Unsupported type: {}!", typ),
                        },
                    });
                    i += 4;
                } else {
                    panic!("Invalid stack declaration");
                }
            }
            Token::Out => {
                if let Token::Assign = tokens[i + 1] {
                    if let Token::Identifier(name) = &tokens[i + 2] {
                        exprs.push(Expr::Output(Box::new(Expr::Identifier(name.clone()))));
                        i += 3;
                    } else if let Token::StringLiteral(val) = &tokens[i + 2] {
                        exprs.push(Expr::Output(Box::new(Expr::StringLiteral(val.into()))));
                        i += 3;
                    } else if let Token::NumberLiteral(val) = &tokens[i + 2] {
                        exprs.push(Expr::Output(Box::new(Expr::NumberLiteral(*val))));
                        i += 3;
                    } else {
                        panic!("Expected identifier/value after out ->");
                    }
                } else {
                    panic!("Expected <- after out");
                }
            }
            Token::In => {
                if let Token::Get = tokens[i + 1] {
                    if let Token::Identifier(name) = &tokens[i + 2] {
                        exprs.push(Expr::Input(Box::new(Expr::Identifier(name.clone()))));
                        i += 3;
                    } else {
                        panic!("Expected identifier after in ->");
                    }
                } else {
                    panic!("Expected -> after out");
                }
            }

            _ => panic!("Unknown token: {:?}", tokens[i]),
        }
    }
    exprs
}
