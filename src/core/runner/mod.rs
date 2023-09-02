use crate::core::{
    lexer::lexer::Lexer,
    parser::{ast::Node, parser::Parser},
};

pub mod file_runner;
pub mod repl;

fn print_parse_errors(errors: &Vec<String>) {
    println!("Woops! We ran into some issue here!");
    println!(" parser errors:");
    for err in errors {
        println!("\t{}", err)
    }
}

fn exec(input: &str) -> String {
    let l = Lexer::new(input.trim().to_string());
    let mut p = Parser::new(l);
    let program = p.parse_program().unwrap();
    if p.errors().len() != 0 {
        print_parse_errors(p.errors());
        return "".to_string();
    }
    program.string()
}
