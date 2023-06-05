use std::io;

use crate::{
    lexer::lexer::Lexer,
    parser::{ast::Node, parser::Parser},
};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut input = String::new();
    loop {
        println!("{}", PROMPT);
        io::stdin().read_line(&mut input).unwrap();
        let mut l = Lexer::new(input.trim().to_string());
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        if p.errors().len() != 0 {
            print_parse_errors(p.errors());
            continue;
        }

        println!("{}", program.string());

        input = String::new();
    }
}

fn print_parse_errors(errors: &Vec<String>) {
    for err in errors {
        println!("\t{}", err)
    }
}
