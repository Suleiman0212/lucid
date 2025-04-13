#![allow(unused)]

use crate::ast::Expr;
use core::panic;
use std::{collections::HashMap, io::Write};

pub fn interpret(exprs: Vec<Expr>) {
    let mut stack_vars: HashMap<String, String> = HashMap::new();

    for expr in exprs {
        match expr {
            Expr::StackDecl { name, typ, value } => match typ.as_str() {
                "text" => {
                    if let Expr::StringLiteral(val) = *value {
                        stack_vars.insert(name, val);
                    } else {
                        stack_vars.insert(name, "".into());
                    }
                }
                "num" => {
                    if let Expr::NumberLiteral(val) = *value {
                        stack_vars.insert(name, val.to_string());
                    } else {
                        panic!("Expected number literal in assignemnt!");
                    }
                }
                _ => panic!("Unsupported type: {}!", typ),
            },
            Expr::Output(inner) => match *inner {
                Expr::Identifier(name) => {
                    use std::io::stdout;
                    if let Some(val) = stack_vars.get(&name) {
                        print!("{}", val);
                        let val = val.replace("\\n", "\n");
                        stdout().flush().unwrap();
                    } else {
                        panic!("Variable not found: {}!", name);
                    }
                }
                Expr::StringLiteral(val) => {
                    use std::io::stdout;
                    let val = val.replace("\\n", "\n");
                    print!("{}", val);
                    stdout().flush().unwrap();
                }
                _ => panic!("Unsupported output expression!"),
            },
            Expr::Input(inner) => match *inner {
                Expr::Identifier(name) => {
                    use std::io::stdin;
                    if let Some(val) = stack_vars.get(&name) {
                        let mut buf = String::new();
                        stdin().read_line(&mut buf).unwrap();
                        stack_vars.insert(name, buf).unwrap();
                    } else {
                        panic!("Variable not found: {}!", name);
                    }
                }
                _ => panic!("Unsupported input expression!"),
            },
            _ => panic!("Unsupported expression!"),
        }
    }
}
