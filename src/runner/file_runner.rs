use std::fs;

use super::exec;

pub fn run_file(file_name: &str) {
    // TODO: reads file and exec
    let file_content =
        fs::read_to_string(file_name).expect("Should have been able to read content of file");

    exec(&file_content);
}
