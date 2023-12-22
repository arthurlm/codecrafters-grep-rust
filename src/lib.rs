mod error;

pub use error::*;

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Pattern {
    Text(String),
    Digit,
    Chars,
}

impl Pattern {
    pub fn matches(&self, input_line: &str) -> bool {
        match self {
            Pattern::Text(expected_text) => input_line.contains(expected_text),
            Pattern::Digit => input_line.chars().any(|x| x.is_digit(10)),
            Pattern::Chars => input_line.chars().any(|x| x.is_ascii_alphanumeric()),
        }
    }
}

impl FromStr for Pattern {
    type Err = GrepError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            r"\d" => Ok(Pattern::Digit),
            r"\w" => Ok(Pattern::Chars),
            x if x.is_empty() => Err(GrepError::InvalidPattern),
            _ => Ok(Pattern::Text(input.to_string())),
        }
    }
}
