use std::io::{self, Write};

static ANSI_RESET: &str = "[0;0m";
static ANSI_GREEN_BOLD: &str = "[32;1m";

pub fn log_interactive(args: &str) {
    print!("{ANSI_GREEN_BOLD}{args}{ANSI_RESET}");
    io::stdout().flush().unwrap();
}
