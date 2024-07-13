use std::io::{BufRead, BufReader};

pub mod colors;

pub fn read_stdin() -> String{
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read input line");
    line.trim().to_string()
}