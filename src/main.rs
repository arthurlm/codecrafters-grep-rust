use std::{env, io, process};

use grep_starter_rust::*;

fn match_pattern(input_line: &str, input_pattern: &str) -> bool {
    let re = Regexp::parse(input_pattern).expect("Unhandled pattern");
    re.matches(input_line)
}

// Usage: echo <input_text> | your_grep.sh -E <pattern>
fn main() {
    if env::args().nth(1).unwrap() != "-E" {
        eprintln!("Expected first argument to be '-E'");
        process::exit(1);
    }

    let pattern = env::args().nth(2).unwrap();
    let mut input_line = String::new();

    io::stdin().read_line(&mut input_line).unwrap();

    if match_pattern(&input_line, &pattern) {
        process::exit(0)
    } else {
        process::exit(1)
    }
}
