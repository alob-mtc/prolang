use std::env;
mod lexer;
mod repl;
mod token;

fn main() {
    let usr = env::var("USER").unwrap();
    println!("Hello {}! This is the Protolang programing language!", usr);
    println!("Feel free to type in commands");
    repl::start();
}
