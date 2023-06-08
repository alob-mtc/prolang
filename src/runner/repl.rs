use std::{
    env,
    io::{self, Write},
};

use super::exec;

const PROMPT: &str = ">> ";

pub fn start() {
    let usr = env::var("USER").unwrap();
    println!("Hello {}! This is the Prolang programing language!", usr);
    println!("Feel free to type in commands");

    let mut input = String::new();
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        exec(&input);
        input = String::new();
    }
}
