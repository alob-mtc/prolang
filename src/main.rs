use crate::core::runner::{file_runner, repl};
use clap::{Parser, Subcommand};

mod core;

/// The `Prolang` Interpreter CLI.
#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct ProlangCLI {
    // CLI args
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Runs the `Prolang` file provided.
    Run { file_path: String },
    /// Init an interactive repl session
    Repl,
}

fn main() {
    let prolang_cli = ProlangCLI::parse();

    match prolang_cli.command {
        Command::Run { file_path } => file_runner::run_file(file_path),
        Command::Repl => repl::start(),
    }
}
