use std::{env::args, fs::File, io::Read, path::PathBuf};

use interpreter::interpret;
use lexer::Lexer;

mod ast;
mod interpreter;
mod lexer;

fn main() {
    let args: Vec<String> = args().collect();
    let code = read_file(args[1].clone().into());

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.tokenize();
    let exprs = ast::parse(tokens);

    interpret(exprs);
}

fn read_file(path: PathBuf) -> String {
    let mut file = File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}
