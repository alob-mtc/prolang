use std::env::args;

use crate::runner::{file_runner, repl};
mod lexer;
mod parser;
mod runner;

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() > 2 {
        println!("Usge: prolang [script]")
    } else if args.len() == 2 {
        file_runner::run_file(&args[1])
    } else {
        repl::start();
    }
}
