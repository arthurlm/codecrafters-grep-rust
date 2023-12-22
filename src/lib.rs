mod error;

pub use error::*;

#[derive(Debug, PartialEq, Eq)]
pub enum Pattern {
    Text(String),
    Digit,
    Chars,
    PositiveCharGroup(Vec<char>),
    NegativeCharGroup(Vec<char>),
}

impl Pattern {
    pub fn parse(input: &str) -> Result<(&str, Self), GrepError> {
        if input.starts_with(r"\d") {
            Ok((&input[2..], Self::Digit))
        } else if input.starts_with(r"\w") {
            Ok((&input[2..], Self::Chars))
        } else if input.starts_with(r"[") {
            match input.chars().position(|c| c == ']') {
                None => Err(GrepError::InvalidPattern),
                Some(end) => {
                    let sub_input = &input[1..end];
                    let mut chars: Vec<_> = sub_input.chars().collect();
                    if chars.first().copied() == Some('^') {
                        Ok((
                            &input[end + 1..],
                            Self::NegativeCharGroup(chars.drain(1..).collect()),
                        ))
                    } else {
                        Ok((&input[end + 1..], Self::PositiveCharGroup(chars)))
                    }
                }
            }
        } else if input.is_empty() {
            Err(GrepError::InvalidPattern)
        } else {
            Ok(("", Self::Text(input.to_string())))
        }
    }

    pub fn matches(&self, input_line: &str) -> bool {
        match self {
            Self::Text(expected_text) => input_line.contains(expected_text),
            Self::Digit => input_line.chars().any(|x| x.is_digit(10)),
            Self::Chars => input_line.chars().any(|x| x.is_ascii_alphanumeric()),
            Self::PositiveCharGroup(values) => input_line.chars().any(|x| values.contains(&x)),
            Self::NegativeCharGroup(values) => input_line.chars().all(|x| !values.contains(&x)),
        }
    }
}
