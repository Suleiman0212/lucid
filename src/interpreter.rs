#![allow(unused)]

use crate::ast::Expr;
use std::collections::HashMap;
use std::io::{Write, stdin, stdout};

fn handle_stack_decl(
    stack_vars: &mut HashMap<String, String>,
    name: String,
    typ: String,
    value: Expr,
) {
    match typ.as_str() {
        "text" => {
            let val = if let Expr::StringLiteral(val) = value {
                val
            } else {
                "".to_string()
            };
            stack_vars.insert(name, val);
        }
        "num" => {
            let val = if let Expr::NumberLiteral(val) = value {
                val.to_string()
            } else {
                panic!("Expected number literal in assignment!");
            };
            stack_vars.insert(name, val);
        }
        _ => panic!("Unsupported type: {}", typ),
    };
}

fn handle_output(stack_vars: &HashMap<String, String>, expr: Expr) {
    let result = evaluate_expr(stack_vars, expr);
    print!("{}", result);
    stdout().flush().unwrap();
}

fn evaluate_expr(stack_vars: &HashMap<String, String>, expr: Expr) -> String {
    match expr {
        Expr::Identifier(name) => stack_vars
            .get(&name)
            .cloned()
            .unwrap_or_else(|| panic!("Variable not found: {}", name)),
        Expr::StringLiteral(val) => val.replace("\\n", "\n"),
        Expr::NumberLiteral(val) => val.to_string(),
        Expr::Addition { left, right } => {
            let left_val = evaluate_expr(stack_vars, *left);
            let right_val = evaluate_expr(stack_vars, *right);
            format!("{}{}", left_val, right_val)
        }
        _ => panic!("Unsupported expression in evaluation: {:?}", expr),
    }
}

fn handle_input(stack_vars: &mut HashMap<String, String>, expr: Expr) {
    if let Expr::Identifier(name) = expr {
        let mut buf = String::new();
        stdin().read_line(&mut buf).unwrap();
        stack_vars.insert(name, buf.trim().to_string());
    } else {
        panic!("Input can only go into identifier!");
    }
}

pub fn interpret(exprs: Vec<Expr>) {
    let mut stack_vars: HashMap<String, String> = HashMap::new();

    for expr in exprs {
        match expr {
            Expr::StackDecl { name, typ, value } => {
                handle_stack_decl(&mut stack_vars, name, typ, *value)
            }
            Expr::Output(expr) => handle_output(&stack_vars, *expr),
            Expr::Input(expr) => handle_input(&mut stack_vars, *expr),
            _ => panic!("Unsupported expression in interpreter: {:?}", expr),
        }
    }
}
