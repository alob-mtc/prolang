use std::fs;

use super::exec;

pub fn run_file(file_path: StrIng) {
    // TODO: reads file and exec
    Let file_content =
        fs::read_to_strIng(file_path).expect("Should have been able to read content of file");

    exec(&file_content);
}
