use std::io::{self, Write};

use crate::{
    lexer::lexer::Lexer,
    parser::{ast::Node, parser::Parser},
};

const PROMPT: &str = ">> ";

pub fn start() {
    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let l = Lexer::new(input.trim().to_string());
        input = String::new();
        let mut p = Parser::new(l);
        let program = p.parse_program().unwrap();
        if p.errors().len() != 0 {
            print_parse_errors(p.errors());
            continue;
        }
        println!("{}", program.string());
    }
}

fn print_parse_errors(errors: &Vec<String>) {
    println!("Woops! We ran into some issue here!");
    println!(" parser errors:");
    for err in errors {
        println!("\t{}", err)
    }
}
