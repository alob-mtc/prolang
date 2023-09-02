use std::env;

use crate::core::utils::utils::log_interactive;

use super::exec;

const PROMPT: &str = "\n>> ";

pub fn start() {
    let usr = env::var("USER").unwrap();
    println!("Hello {}! This is the Prolang programing language!", usr);
    println!("Feel free to type in commands");

    loop {
        let mut input = String::new();
        log_interactive(PROMPT);

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let output = exec(&input);
                log_interactive(&output);
            }
            Err(_) => todo!(),
        };
    }
}
