use std::fs::read_to_string;
use std::io;

pub fn read_stdin() -> Vec<String> {
    io::stdin().lines().filter_map(Result::ok).collect()
}

#[allow(dead_code)]
pub fn read_file(path: &str) -> Vec<String> {
    let file_contents = read_to_string(path);

    match file_contents {
        Err(_) => panic!("Input file {path} does not exist."),
        Ok(s) => s.lines().map(String::from).collect(),
    }
}

#[allow(dead_code)]
fn main() {}
